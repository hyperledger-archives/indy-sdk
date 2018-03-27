//
//  Indy_demoTests.m
//  Indy-demoTests
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>
#import "TestUtils.h"
#import "WalletUtils.h"
#import "NSDictionary+JSON.h"
#import "AnoncredsUtils.h"

@interface AnoncredsDemo : XCTestCase

@end

@implementation AnoncredsDemo

- (void)setUp {
    [super setUp];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

- (void)testAnoncredsDemo {
    [TestUtils cleanupStorage];

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    NSError *ret;

    // 1. Create wallet
    NSString *walletName = @"issuer_wallet";
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:[TestUtils pool]
                                                      walletName:walletName
                                                           xtype:[TestUtils defaultType]
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createWalletWithPoolName() failed!");

    // 2. Open wallet
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName
                                                    config:nil
                                                 outHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::openWalletWithName() failed!");

    //3. Issuer create Schema
    __block NSString *schemaId;
    __block NSString *schemaJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateSchemaWithName:[TestUtils gvtSchemaName]
                                                              version:[TestUtils schemaVersion]
                                                                attrs:[TestUtils gvtSchemaAttrs]
                                                            issuerDID:[TestUtils issuerDid]
                                                             schemaId:&schemaId
                                                           schemaJson:&schemaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateSchemaForIssuerDID failed");

    XCTAssertTrue([schemaId isValid], @"invalid schemaId: %@", schemaId);
    XCTAssertTrue([schemaJson isValid], @"invalid schemaJson: %@", schemaJson);

    // 4. Issuer create Credential Definition for Schema
    __block NSString *credentialDefId;
    __block NSString *credentialDefJSON;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateAndStoreCredentialDefForSchema:schemaJson
                                                                            issuerDID:[TestUtils issuerDid]
                                                                                  tag:[TestUtils tag]
                                                                                 type:nil
                                                                           configJSON:[[AnoncredsUtils sharedInstance] defaultCredentialDefConfig]
                                                                         walletHandle:walletHandle
                                                                            credDefId:&credentialDefId
                                                                          credDefJson:&credentialDefJSON];
    XCTAssertEqual(ret.code, Success, @"issuerCreateCredentialDefinitionWithWalletHandle failed");

    XCTAssertTrue([credentialDefId isValid], @"invalid credentialDefId: %@", credentialDefId);
    XCTAssertTrue([credentialDefJSON isValid], @"invalid credentialDefJSON: %@", credentialDefJSON);

    // 5. Prover create Master Secret
    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecret:[TestUtils commonMasterSecretName]
                                                       walletHandle:walletHandle
                                                  outMasterSecretId:nil];

    XCTAssertEqual(ret.code, Success, @"proverCreateMasterSecret() failed!");

    // 6. Issuer create Credential Offer
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    __block NSString *credentialOfferJSON = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateCredentialOfferForCredDefId:credentialDefId
                                                                      walletHandle:walletHandle
                                                                     credOfferJson:&credentialOfferJSON];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreCredentialDef() failed!");

    // 7. Prover create Credential Request
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    __block NSString *credentialReqJSON = nil;
    __block NSString *credentialReqMetadataJSON = nil;
    ret = [[AnoncredsUtils sharedInstance] proverCreateCredentialReqForCredentialOffer:credentialOfferJSON
                                                                     credentialDefJSON:credentialDefJSON
                                                                             proverDID:[TestUtils proverDid]
                                                                        masterSecretID:[TestUtils commonMasterSecretName]
                                                                          walletHandle:walletHandle
                                                                           credReqJson:&credentialReqJSON
                                                                   credReqMetadataJson:&credentialReqMetadataJSON];
    XCTAssertEqual(ret.code, Success, @"proverCreateAndStoreCredentialReq() failed!");

    // 8. Issuer create Credential for Credential Request
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    __block NSString *credentialJSON = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateCredentialForCredentialRequest:credentialReqJSON
                                                                        credOfferJSON:credentialOfferJSON
                                                                       credValuesJSON:[[AnoncredsUtils sharedInstance] getGvtCredentialValuesJson]
                                                                             revRegId:nil
                                                              blobStorageReaderHandle:nil
                                                                         walletHandle:walletHandle
                                                                             credJson:&credentialJSON
                                                                          credRevocId:nil
                                                                    revocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, Success, @"issuerCreateCredential() failed!");

    // 9. Prover process and store Credential
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    ret = [[AnoncredsUtils sharedInstance] proverStoreCredential:credentialJSON
                                                          credID:[[AnoncredsUtils sharedInstance] credentialId1]
                                                     credReqJSON:credentialReqJSON
                                             credReqMetadataJSON:credentialReqMetadataJSON
                                                     credDefJSON:credentialDefJSON
                                                   revRegDefJSON:nil
                                                    walletHandle:walletHandle
                                                       outCredId:nil];
    XCTAssertEqual(ret.code, Success, @"proverStoreCredential() failed!");

    // 10. Prover gets Credentials for Proof Request
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    NSString *proofReqJSON = [NSString stringWithFormat:@"\
                              {"
            "\"nonce\":\"123432421212\","
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","
            "\"requested_attributes\":{\
                                    \"attr1_referent\":{\
                                        \"name\":\"name\"\
                                    }\
                              },\
                              \"requested_predicates\":{\
                                    \"predicate1_referent\":{\
                                        \"name\":\"age\",\
                                        \"p_type\":\">=\",\
                                        \"p_value\":18\
                                    }\
                              }\
                            }"];

    __block NSString *credentialsJson = nil;

    ret = [[AnoncredsUtils sharedInstance] proverGetCredentialsForProofReq:proofReqJSON
                                                              walletHandle:walletHandle
                                                           credentialsJson:&credentialsJson];
    XCTAssertEqual(ret.code, Success, @"proverGetCredentialsForProofReq() failed!");

    NSDictionary *credentials = [NSDictionary fromString:credentialsJson];
    XCTAssertTrue(credentials, @"serialization failed");

    NSDictionary *credentials_for_attr_1 = credentials[@"attrs"][@"attr1_referent"][0];
    XCTAssertTrue(credentials_for_attr_1, @"no object for key \"attr1_referent\"");
    NSString *credentialReferent = credentials_for_attr_1[@"cred_info"][@"referent"];

    // 11. Prover create Proof for Proof Request
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    NSString *requestedCredentialsJSON = [NSString stringWithFormat:@"{\
                                     \"self_attested_attributes\":{},\
                                     \"requested_attributes\":{\"attr1_referent\":{\"cred_id\":\"%@\",\"revealed\":true}},\
                                     \"requested_predicates\":{\"predicate1_referent\":{\"cred_id\":\"%@\"}}\
                                     }", credentialReferent, credentialReferent];

    NSString *schemasJson = [NSString stringWithFormat:@"{\"%@\":%@}", schemaId, schemaJson];
    NSString *credentialDefsJson = [NSString stringWithFormat:@"{\"%@\":%@}", credentialDefId, credentialDefJSON];
    NSString *revocStatesJson = @"{}";

    NSString *proofJSON = nil;
    ret = [[AnoncredsUtils sharedInstance] proverCreateProofForRequest:proofReqJSON
                                              requestedCredentialsJSON:requestedCredentialsJSON
                                                        masterSecretID:[TestUtils commonMasterSecretName]
                                                           schemasJSON:schemasJson
                                                    credentialDefsJSON:credentialDefsJson
                                                       revocStatesJSON:revocStatesJson
                                                          walletHandle:walletHandle
                                                             proofJson:&proofJSON];
    XCTAssertEqual(ret.code, Success, @"proverCreateProof() failed!");

    // 12. Verifier verify proof
    NSDictionary *proof = [NSDictionary fromString:proofJSON];
    XCTAssertTrue(proof, @"serialization failed");

    NSDictionary *revealedAttr1 = proof[@"requested_proof"][@"revealed_attrs"][@"attr1_referent"];

    NSString *revocRegDefsJson = @"{}";
    NSString *revocRegsJson = @"{}";


    BOOL valid = false;
    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProofRequest:proofReqJSON
                                                            proofJSON:proofJSON
                                                          schemasJSON:schemasJson
                                                   credentialDefsJSON:credentialDefsJson
                                                     revocRegDefsJSON:revocRegsJson
                                                        revocRegsJSON:revocRegsJson
                                                              isValid:&valid];
    XCTAssertEqual(ret.code, Success, @"verifierVerifyProof() failed!");
    XCTAssertTrue(valid);

    // 12. close wallet
    ret = [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"closeWallet() failed!");

    [TestUtils cleanupStorage];
}

- (void)testAnoncredsDemoForKeychainWallet {
    [TestUtils cleanupStorage];
    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    NSError *ret;

    // 0. register wallet type
    NSString *xType = @"keychain";
    ret = [[WalletUtils sharedInstance] registerWalletType:xType];

    // 1. Create wallet
    NSString *walletName = @"issuer_wallet";
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:[TestUtils pool]
                                                      walletName:walletName
                                                           xtype:xType
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createWalletWithPoolName() failed!");

    // 2. Open wallet
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName
                                                    config:nil
                                                 outHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::openWalletWithName() failed!");

    //3. Issuer create Schema
    //3. Issuer create Schema
    __block NSString *schemaId;
    __block NSString *schemaJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateSchemaWithName:[TestUtils gvtSchemaName]
                                                              version:[TestUtils schemaVersion]
                                                                attrs:[TestUtils gvtSchemaAttrs]
                                                            issuerDID:[TestUtils issuerDid]
                                                             schemaId:&schemaId
                                                           schemaJson:&schemaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateSchemaForIssuerDID failed");

    XCTAssertTrue([schemaId isValid], @"invalid schemaId: %@", schemaId);
    XCTAssertTrue([schemaJson isValid], @"invalid schemaJson: %@", schemaJson);

    // 4. Issuer create Credential Definition for Schema
    __block NSString *credentialDefId;
    __block NSString *credentialDefJSON;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateAndStoreCredentialDefForSchema:schemaJson
                                                                            issuerDID:[TestUtils issuerDid]
                                                                                  tag:[TestUtils tag]
                                                                                 type:nil
                                                                           configJSON:[[AnoncredsUtils sharedInstance] defaultCredentialDefConfig]
                                                                         walletHandle:walletHandle
                                                                            credDefId:&credentialDefId
                                                                          credDefJson:&credentialDefJSON];
    XCTAssertEqual(ret.code, Success, @"issuerCreateCredentialDefinitionWithWalletHandle failed");

    XCTAssertTrue([credentialDefId isValid], @"invalid credentialDefId: %@", credentialDefId);
    XCTAssertTrue([credentialDefJSON isValid], @"invalid credentialDefJSON: %@", credentialDefJSON);

    // 5. Prover create Master Secret
    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecret:[TestUtils commonMasterSecretName]
                                                       walletHandle:walletHandle
                                                               outMasterSecretId:nil];
    XCTAssertEqual(ret.code, Success, @"proverCreateMasterSecret() failed!");

    // 6. Issuer create Credential Offer
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    __block NSString *credentialOfferJSON = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateCredentialOfferForCredDefId:credentialDefId
                                                                      walletHandle:walletHandle
                                                                     credOfferJson:&credentialOfferJSON];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreCredentialDef() failed!");

    // 7. Prover create Credential Request
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    __block NSString *credentialReqJSON = nil;
    __block NSString *credentialReqMetadataJSON = nil;
    ret = [[AnoncredsUtils sharedInstance] proverCreateCredentialReqForCredentialOffer:credentialOfferJSON
                                                                     credentialDefJSON:credentialDefJSON
                                                                             proverDID:[TestUtils proverDid]
                                                                        masterSecretID:[TestUtils commonMasterSecretName]
                                                                          walletHandle:walletHandle
                                                                           credReqJson:&credentialReqJSON
                                                                   credReqMetadataJson:&credentialReqMetadataJSON];
    XCTAssertEqual(ret.code, Success, @"proverCreateAndStoreCredentialReq() failed!");

    // 8. Issuer create Credential for Credential Request
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    __block NSString *credentialJSON = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateCredentialForCredentialRequest:credentialReqJSON
                                                                        credOfferJSON:credentialOfferJSON
                                                                       credValuesJSON:[[AnoncredsUtils sharedInstance] getGvtCredentialValuesJson]
                                                                             revRegId:nil
                                                              blobStorageReaderHandle:nil
                                                                         walletHandle:walletHandle
                                                                             credJson:&credentialJSON
                                                                          credRevocId:nil
                                                                    revocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, Success, @"issuerCreateCredential() failed!");

    // 9. Prover process and store Credential
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    ret = [[AnoncredsUtils sharedInstance] proverStoreCredential:credentialJSON
                                                          credID:[[AnoncredsUtils sharedInstance] credentialId1]
                                                     credReqJSON:credentialReqJSON
                                             credReqMetadataJSON:credentialReqMetadataJSON
                                                     credDefJSON:credentialDefJSON
                                                   revRegDefJSON:nil
                                                    walletHandle:walletHandle
                                                       outCredId:nil];
    XCTAssertEqual(ret.code, Success, @"proverStoreCredential() failed!");

    // 10. Prover gets Credentials for Proof Request
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    NSString *proofReqJSON = [NSString stringWithFormat:@"\
                              {"
            "\"nonce\":\"123432421212\","
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","
            "\"requested_attributes\":{\
                                    \"attr1_referent\":{\
                                        \"name\":\"name\"\
                                    }\
                              },\
                              \"requested_predicates\":{\
                                    \"predicate1_referent\":{\
                                        \"name\":\"age\",\
                                        \"p_type\":\">=\",\
                                        \"p_value\":18\
                                    }\
                              }\
                            }"];

    __block NSString *credentialsJson = nil;

    ret = [[AnoncredsUtils sharedInstance] proverGetCredentialsForProofReq:proofReqJSON
                                                              walletHandle:walletHandle
                                                           credentialsJson:&credentialsJson];
    XCTAssertEqual(ret.code, Success, @"proverGetCredentialsForProofReq() failed!");

    NSDictionary *credentials = [NSDictionary fromString:credentialsJson];
    XCTAssertTrue(credentials, @"serialization failed");

    NSDictionary *credentials_for_attr_1 = credentials[@"attrs"][@"attr1_referent"][0];
    XCTAssertTrue(credentials_for_attr_1, @"no object for key \"attr1_referent\"");
    NSString *credentialReferent = credentials_for_attr_1[@"cred_info"][@"referent"];

    // 11. Prover create Proof for Proof Request
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    NSString *requestedCredentialsJSON = [NSString stringWithFormat:@"{\
                                     \"self_attested_attributes\":{},\
                                     \"requested_attributes\":{\"attr1_referent\":{\"cred_id\":\"%@\",\"revealed\":true}},\
                                     \"requested_predicates\":{\"predicate1_referent\":{\"cred_id\":\"%@\"}}\
                                     }", credentialReferent, credentialReferent];

    NSString *schemasJson = [NSString stringWithFormat:@"{\"%@\":%@}", schemaId, schemaJson];
    NSString *credentialDefsJson = [NSString stringWithFormat:@"{\"%@\":%@}", credentialDefId, credentialDefJSON];
    NSString *revocStatesJson = @"{}";

    NSString *proofJSON = nil;
    ret = [[AnoncredsUtils sharedInstance] proverCreateProofForRequest:proofReqJSON
                                              requestedCredentialsJSON:requestedCredentialsJSON
                                                        masterSecretID:[TestUtils commonMasterSecretName]
                                                           schemasJSON:schemasJson
                                                    credentialDefsJSON:credentialDefsJson
                                                       revocStatesJSON:revocStatesJson
                                                          walletHandle:walletHandle
                                                             proofJson:&proofJSON];
    XCTAssertEqual(ret.code, Success, @"proverCreateProof() failed!");

    // 12. Verifier verify proof
    NSDictionary *proof = [NSDictionary fromString:proofJSON];
    XCTAssertTrue(proof, @"serialization failed");

    NSDictionary *revealedAttr1 = proof[@"requested_proof"][@"revealed_attrs"][@"attr1_referent"];

    NSString *revocRegDefsJson = @"{}";
    NSString *revocRegsJson = @"{}";

    BOOL valid = false;

    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProofRequest:proofReqJSON
                                                            proofJSON:proofJSON
                                                          schemasJSON:schemasJson
                                                   credentialDefsJSON:credentialDefsJson
                                                     revocRegDefsJSON:revocRegsJson
                                                        revocRegsJSON:revocRegsJson
                                                              isValid:&valid];
    XCTAssertEqual(ret.code, Success, @"verifierVerifyProof() failed!");
    XCTAssertTrue(valid);

    // 13. close wallet

    ret = [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"closeWallet() failed!");

    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
    [TestUtils cleanupStorage];
}

@end
