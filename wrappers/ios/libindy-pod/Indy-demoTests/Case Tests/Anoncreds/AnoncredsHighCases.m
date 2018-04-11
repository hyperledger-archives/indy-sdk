//
//  AnoncredsHighCase.m
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

@interface AnoncredsHignCases : XCTestCase

@end

@implementation AnoncredsHignCases

- (void)setUp {
    [super setUp];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

// MARK: - Issuer create and store credential def

- (void)testIssuerCreateAndStoreCredentialDefWorks {
    NSError *ret;

    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. Issuer create Schema
    NSString *schemaJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateSchemaWithName:@"other_schema"
                                                              version:[TestUtils schemaVersion]
                                                                attrs:[TestUtils gvtSchemaAttrs]
                                                            issuerDID:[TestUtils issuerDid]
                                                             schemaId:nil
                                                           schemaJson:&schemaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateSchemaForIssuerDID failed");

    // 3. issuer create credential definition
    NSString *credentialDefId;
    NSString *credentialDefJSON;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateAndStoreCredentialDefForSchema:schemaJson
                                                                            issuerDID:[TestUtils issuerDid]
                                                                                  tag:@"Works"
                                                                                 type:nil
                                                                           configJSON:[[AnoncredsUtils sharedInstance] defaultCredentialDefConfig]
                                                                         walletHandle:walletHandle
                                                                            credDefId:&credentialDefId
                                                                          credDefJson:&credentialDefJSON];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateCredentialDefinifionWithWalletHandle failed");
    XCTAssertTrue([credentialDefJSON isValid], @"invalid credentialDefJSON: %@", credentialDefJSON);
}

- (void)testIssuerCreateAndStoreCredentialDefWorksForInvalidWallet {

    NSError *ret;

    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. Create credential definition
    IndyHandle invalidWalletHandle = walletHandle + 100;
    NSString *schemaJson = [[AnoncredsUtils sharedInstance] getGvtSchemaJson];

    NSString *credentialDefId;
    NSString *credentialDefJSON;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateAndStoreCredentialDefForSchema:schemaJson
                                                                            issuerDID:[TestUtils issuerDid]
                                                                                  tag:[TestUtils tag]
                                                                                 type:nil
                                                                           configJSON:[[AnoncredsUtils sharedInstance] defaultCredentialDefConfig]
                                                                         walletHandle:invalidWalletHandle
                                                                            credDefId:&credentialDefId
                                                                          credDefJson:&credentialDefJSON];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"AnoncredsUtils::issuerCreateCredentialDefinifionWithWalletHandle failed: returned wrong error code");
}

// MARK: - Prover create master secret

- (void)testProverCreateMasterSecretWorks {

    NSError *ret;
    IndyHandle walletHandle = 0;

    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed with code:%ld", (long) ret.code);

    // 2. create master secret
    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecret:@"master_secret_name1"
                                                       walletHandle:walletHandle
                                                  outMasterSecretId:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateMasterSecret failed with code:%ld", (long) ret.code);

}

- (void)testProverCreateMasterSecretWorksInvalidWalletHandle {

    NSError *ret;
    IndyHandle walletHandle = 0;

    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed with code:%ld", (long) ret.code);

    // 2. create master secret
    IndyHandle invalidWalletHandle = walletHandle + 1;
    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecret:@"master_secret_name2"
                                                       walletHandle:invalidWalletHandle
                                                               outMasterSecretId:nil];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"AnoncredsUtils::proverCreateMasterSecret returned not WalletInvalidHandle code:%ld", (long) ret.code);

}

// MARK: - Prover create credential request
- (void)testProverCreateCredentialRequestWorks {

    NSError *ret;
    IndyHandle walletHandle = 0;
    NSString *credentialDef;

    // 1. get wallet handle
    NSString *credentialOffer;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:&credentialDef
                                                             credentialOfferJson:&credentialOffer
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed with code:%ld", (long) ret.code);

    // 2. get credential request
    NSString *credentialRequestJson;
    NSString *credentialRequestMetadataJson;
    ret = [[AnoncredsUtils sharedInstance] proverCreateCredentialReqForCredentialOffer:credentialOffer
                                                                     credentialDefJSON:credentialDef
                                                                             proverDID:[TestUtils proverDid]
                                                                        masterSecretID:[TestUtils commonMasterSecretName]
                                                                          walletHandle:walletHandle
                                                                           credReqJson:&credentialRequestJson
                                                                   credReqMetadataJson:&credentialRequestMetadataJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreCredentialReq failed with code:%ld", (long) ret.code);
    XCTAssertTrue([credentialRequestJson isValid], @"invalid credentialRequestJson: %@", credentialRequestJson);
    XCTAssertTrue([credentialRequestMetadataJson isValid], @"invalid credentialRequestMetadataJson: %@", credentialRequestMetadataJson);
}

- (void)testProverCreateCredentialReqWorksForInvalidWallet {

    NSError *ret;
    IndyHandle walletHandle = 0;
    NSString *credentialDef;
    NSString *credentialOffer;

    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:&credentialDef
                                                             credentialOfferJson:&credentialOffer
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. create and store credential requets
    IndyHandle invalidWalletHandle = walletHandle + 1;
    NSString *credentialRequestJson;
    NSString *credentialRequestMetadataJson;
    ret = [[AnoncredsUtils sharedInstance] proverCreateCredentialReqForCredentialOffer:credentialOffer
                                                                     credentialDefJSON:credentialDef
                                                                             proverDID:[TestUtils proverDid]
                                                                        masterSecretID:[TestUtils commonMasterSecretName]
                                                                          walletHandle:invalidWalletHandle
                                                                           credReqJson:&credentialRequestJson
                                                                   credReqMetadataJson:&credentialRequestMetadataJson];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"AnoncredsUtils::proverCreateAndStoreCredentialReq failed");

}

// MARK: - Issuer create credential

- (void)testIssuerCreateCredentialWorks {

    NSError *ret;
    IndyHandle walletHandle = 0;

    // 1. get wallet handle and credential request
    NSString *credentialRequest;
    NSString *credentialOffer;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:nil
                                                             credentialOfferJson:&credentialOffer
                                                               credentialReqJson:&credentialRequest
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    NSString *credentialJson = [[AnoncredsUtils sharedInstance] getGvtCredentialValuesJson];
    ret = [[AnoncredsUtils sharedInstance] issuerCreateCredentialForCredentialRequest:credentialRequest
                                                                        credOfferJSON:credentialOffer
                                                                       credValuesJSON:[[AnoncredsUtils sharedInstance] getGvtCredentialValuesJson]
                                                                             revRegId:nil
                                                              blobStorageReaderHandle:nil
                                                                         walletHandle:walletHandle
                                                                             credJson:&credentialJson
                                                                          credRevocId:nil
                                                                    revocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateCredentialWithWalletHandle failed");
    XCTAssertTrue([credentialJson isValid], @"invalid credentialJson: %@", credentialJson);
}

- (void)testIssuerCreateCredentialWorksForCredentialDoesNotCorrespondToCredentialValues {

    NSError *ret;
    IndyHandle walletHandle = 0;

    // 1. get wallet handle
    NSString *credentialRequest;
    NSString *credentialOffer;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:nil
                                                             credentialOfferJson:&credentialOffer
                                                               credentialReqJson:&credentialRequest
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. create credential
    NSString *credentialJson = [[AnoncredsUtils sharedInstance] getXyzCredentialValuesJson];
    ret = [[AnoncredsUtils sharedInstance] issuerCreateCredentialForCredentialRequest:credentialRequest
                                                                        credOfferJSON:credentialOffer
                                                                       credValuesJSON:[[AnoncredsUtils sharedInstance] getXyzCredentialValuesJson]
                                                                             revRegId:nil
                                                              blobStorageReaderHandle:nil
                                                                         walletHandle:walletHandle
                                                                             credJson:&credentialJson
                                                                          credRevocId:nil
                                                                    revocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::issuerCreateCredentialWithWalletHandle returned wrong code");
}

- (void)testIssuerCreateCredentialWorksForInvalidWalletHandle {
    NSError *ret;
    IndyHandle walletHandle = 0;

    // 1. get wallet handle
    NSString *credentialRequest;
    NSString *credentialOffer;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:nil
                                                             credentialOfferJson:&credentialOffer
                                                               credentialReqJson:&credentialRequest
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. create credential
    NSString *credentialJson = [[AnoncredsUtils sharedInstance] getGvtCredentialValuesJson];

    IndyHandle invalidWalletHandle = walletHandle + 1;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateCredentialForCredentialRequest:credentialRequest
                                                                        credOfferJSON:credentialOffer
                                                                       credValuesJSON:[[AnoncredsUtils sharedInstance] getGvtCredentialValuesJson]
                                                                             revRegId:nil
                                                              blobStorageReaderHandle:nil
                                                                         walletHandle:invalidWalletHandle
                                                                             credJson:&credentialJson
                                                                          credRevocId:nil
                                                                    revocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"AnoncredsUtils::issuerCreateCredentialWithWalletHandle returned wrong error code.");
}

// MARK: - Prover store credential

- (void)testProverStoreCredentialWorks {

    NSError *ret;
    IndyHandle walletHandle = 0;
    NSString *credentialDefJson;
    NSString *credentialOfferJson;

    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:&credentialDefJson
                                                             credentialOfferJson:&credentialOfferJson
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    XCTAssertTrue([credentialDefJson isValid], @"credentialDefJson is wrong:%@", credentialDefJson);

    // 2. get credential request
    NSString *credentialRequest;
    NSString *credentialRequestMetadata;
    ret = [[AnoncredsUtils sharedInstance] proverCreateCredentialReqForCredentialOffer:credentialOfferJson
                                                                     credentialDefJSON:credentialDefJson
                                                                             proverDID:[TestUtils proverDid]
                                                                        masterSecretID:[TestUtils commonMasterSecretName]
                                                                          walletHandle:walletHandle
                                                                           credReqJson:&credentialRequest
                                                                   credReqMetadataJson:&credentialRequestMetadata];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreCredentialReq failed");
    XCTAssertTrue([credentialRequest isValid], @"credentialRequest is wrong:%@", credentialRequest);

    // 4. create credential
    NSString *credentialJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateCredentialForCredentialRequest:credentialRequest
                                                                        credOfferJSON:credentialOfferJson
                                                                       credValuesJSON:[[AnoncredsUtils sharedInstance] getGvtCredentialValuesJson]
                                                                             revRegId:nil
                                                              blobStorageReaderHandle:nil
                                                                         walletHandle:walletHandle
                                                                             credJson:&credentialJson
                                                                          credRevocId:nil
                                                                    revocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateCredentialWithWalletHandle failed");
    XCTAssertTrue([credentialJson isValid], @"credentialJson is wrong:%@", credentialJson);

    // 5. store credential
    ret = [[AnoncredsUtils sharedInstance] proverStoreCredential:credentialJson
                                                          credID:[[AnoncredsUtils sharedInstance] credentialId1]
                                                     credReqJSON:credentialRequest
                                             credReqMetadataJSON:credentialRequestMetadata
                                                     credDefJSON:credentialDefJson
                                                   revRegDefJSON:nil
                                                    walletHandle:walletHandle
                                                       outCredId:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreCredentialWithWalletHandle failed");
}

- (void)testProverStoreCredentialWorksForInvalidWalletHandle {

    NSError *ret;
    IndyHandle walletHandle = 0;
    NSString *credentialDefJson;
    NSString *credentialOfferJson;
    NSString *credentialJson;

    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:&credentialDefJson
                                                             credentialOfferJson:&credentialOfferJson
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    XCTAssertTrue([credentialDefJson isValid], @"credentialDefJson is wrong:%@", credentialDefJson);

    // 2. get credential request
    NSString *credentialRequest;
    NSString *credentialRequestMetadata;
    ret = [[AnoncredsUtils sharedInstance] proverCreateCredentialReqForCredentialOffer:credentialOfferJson
                                                                     credentialDefJSON:credentialDefJson
                                                                             proverDID:[TestUtils proverDid]
                                                                        masterSecretID:[TestUtils commonMasterSecretName]
                                                                          walletHandle:walletHandle
                                                                           credReqJson:&credentialRequest
                                                                   credReqMetadataJson:&credentialRequestMetadata];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreCredentialReq failed");
    XCTAssertTrue([credentialRequest isValid], @"credentialRequest is wrong:%@", credentialRequest);

    // 4. create credential
    ret = [[AnoncredsUtils sharedInstance] issuerCreateCredentialForCredentialRequest:credentialRequest
                                                                        credOfferJSON:credentialOfferJson
                                                                       credValuesJSON:[[AnoncredsUtils sharedInstance] getGvtCredentialValuesJson]
                                                                             revRegId:nil
                                                              blobStorageReaderHandle:nil
                                                                         walletHandle:walletHandle
                                                                             credJson:&credentialJson
                                                                          credRevocId:nil
                                                                    revocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateCredentialWithWalletHandle failed");
    XCTAssertTrue([credentialJson isValid], @"credentialJson is wrong:%@", credentialJson);

    // 5. store credential
    IndyHandle invalidWalletHandle = walletHandle + 1;

    ret = [[AnoncredsUtils sharedInstance] proverStoreCredential:credentialJson
                                                          credID:[[AnoncredsUtils sharedInstance] credentialId1]
                                                     credReqJSON:credentialRequest
                                             credReqMetadataJSON:credentialRequestMetadata
                                                     credDefJSON:credentialDefJson
                                                   revRegDefJSON:nil
                                                    walletHandle:invalidWalletHandle
                                                       outCredId:nil];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"AnoncredsUtils::proverStoreCredentialWithWalletHandle failed");
}

// MARK: - Prover get credentials

- (void)testProverGetCredentialsWorksForEmptyFilter {
    NSError *ret;
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
    XCTAssertTrue([credentialsJson isValid], @"credentialsJson is wrong:%@", credentialsJson);

    NSDictionary *credentialsDict = [NSDictionary fromString:credentialsJson];
    NSArray *credentials = (NSArray *) credentialsDict;

    XCTAssertEqual([credentials count], 3, @"credentials count != 1");
}

- (void)testProverGetCredentialsWorksForFilterByIssuerDid {
    NSError *ret;
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
    XCTAssertTrue([credentialsJson isValid], @"credentialsJson is wrong:%@", credentialsJson);

    NSDictionary *credentialsDict = [NSDictionary fromString:credentialsJson];
    NSArray *credentials = (NSArray *) credentialsDict;

    XCTAssertEqual([credentials count], 1, @"credentials count != 1");
}

- (void)testProverGetCredentialsWorksForFilterByCredentialDefId {
    NSError *ret;
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
    NSString *filter = [NSString stringWithFormat:@"{\"cred_def_id\":\"%@\"}", [[AnoncredsUtils sharedInstance] getIssuer1GvtCredDefId]];
    ret = [[AnoncredsUtils sharedInstance] proverGetCredentialsForFilter:filter
                                                            walletHandle:walletHandle
                                                          credentilsJson:&credentialsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetCredentialsForWalletHandle failed");
    XCTAssertTrue([credentialsJson isValid], @"credentialsJson is wrong:%@", credentialsJson);

    NSDictionary *credentialsDict = [NSDictionary fromString:credentialsJson];
    NSArray *credentials = (NSArray *) credentialsDict;

    XCTAssertEqual([credentials count], 1, @"credentials count != 1");
}

- (void)testProverGetCredentialsWorksForEmptyResult {
    NSError *ret;
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
    XCTAssertTrue([credentialsJson isValid], @"credentialsJson is wrong:%@", credentialsJson);

    NSDictionary *credentialsDict = [NSDictionary fromString:credentialsJson];
    NSArray *credentials = (NSArray *) credentialsDict;

    XCTAssertEqual([credentials count], 0, @"credentials count != 0");
}

- (void)testProverGetCredentialsWorksForInvalidWalletHandle {
    NSError *ret;
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
    IndyHandle invalidWalletHandle = walletHandle + 1;
    ret = [[AnoncredsUtils sharedInstance] proverGetCredentialsForFilter:@"{}"
                                                            walletHandle:invalidWalletHandle
                                                          credentilsJson:&credentialsJson];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"AnoncredsUtils::proverGetCredentialsForWalletHandle returned wrong code");
}

// MARK: - Prover get credentials for proof request

- (void)testProverGetCredentialsForProofReqWorksForRevealedAttr {
    NSError *ret;
    IndyHandle walletHandle = 0;

    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. get credentials
    NSString *proofRequest = [NSString stringWithFormat:@"{"
            "\"nonce\":\"123432421212\","
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","\
        "\"requested_attributes\":{"
            "\"attr1_referent\":{"
            "\"name\":\"name\""
            "}"
            "},"
            "\"requested_predicates\":{}"
            "}"];
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

- (void)testProverGetCredentialsForProofReqWorksForNotFoundAttribute {
    NSError *ret;
    IndyHandle walletHandle = 0;

    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. get credentials
    NSString *proofRequest = @"{"
            "\"nonce\":\"123432421212\","
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","\
        "\"requested_attributes\":{"
            "\"attr1_referent\":{"
            "\"name\":\"some_attr\""
            "}"
            "},"
            "\"requested_predicates\":{}"
            "}";
    NSString *credentialsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetCredentialsForProofReq:proofRequest
                                                              walletHandle:walletHandle
                                                           credentialsJson:&credentialsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetCredentialsForProofReqWithWalletHandle returned wrong code");

    // 3. check credentials
    NSDictionary *credentials = [NSDictionary fromString:credentialsJson];
    XCTAssertEqual([[credentials[@"attrs"] allValues] count], 1, @"attrs length != 1");
    XCTAssertEqual([[credentials[@"predicates"] allValues] count], 0, @"predicates length != 0");
    XCTAssertEqual([credentials[@"attrs"][@"attr1_referent"] count], 0, @"attr1_referent length != 1");
}

- (void)testProverGetCredentialsForProofReqWorksForSatisfyPredicate {
    NSError *ret;
    IndyHandle walletHandle = 0;

    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. get credentials
    NSString *proofRequest = @"{"
            "\"nonce\":\"123432421212\","
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","\
            "\"requested_attributes\":{},"
            "\"requested_predicates\":{\"predicate1_referent\":{\"name\":\"age\",\"p_type\":\">=\",\"p_value\":18}}"
            "}";
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

- (void)testProverGetCredentialsForProofReqWorksForNotSatisfyPredicate {
    NSError *ret;
    IndyHandle walletHandle = 0;

    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. get credentials
    NSString *proofRequest = @"{"
            "\"nonce\":\"123432421212\","
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","\
    "\"requested_attributes\":{},"
            "\"requested_predicates\":{\"predicate1_referent\":{\"name\":\"age\",\"p_type\":\">=\",\"p_value\":58}}"
            "}";
    NSString *credentialsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetCredentialsForProofReq:proofRequest
                                                              walletHandle:walletHandle
                                                           credentialsJson:&credentialsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetCredentialsForProofReqWithWalletHandle returned wrong code");

    // 3. check credentials
    NSDictionary *credentials = [NSDictionary fromString:credentialsJson];
    XCTAssertEqual([[credentials[@"attrs"] allValues] count], 0, @"attrs length != 1");
    XCTAssertEqual([[credentials[@"predicates"] allValues] count], 1, @"predicates length != 0");
    XCTAssertEqual([credentials[@"predicates"][@"predicate1_referent"] count], 0, @"predicate1_referent length != 0");
}


- (void)testProverGetCredentialsForProofReqWorksForMultiplyAttributeAndPredicates {
    NSError *ret;
    IndyHandle walletHandle = 0;

    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. get credentials
    NSString *proofRequest = [NSString stringWithFormat:@"{"
            "\"nonce\":\"123432421212\","
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","
            "\"requested_attributes\":{"
            "\"attr1_referent\":{\"name\":\"name\"},"
            "\"attr2_referent\":{\"name\":\"sex\"}"
            "},"
            "\"requested_predicates\":{"
            "\"predicate1_referent\":{\"name\":\"age\",\"p_type\":\">=\",\"p_value\":18},"
            "\"predicate2_referent\":{\"name\":\"height\",\"p_type\":\">=\",\"p_value\":160}"
            "}}"];
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

- (void)testProverGetCredentialsForProofReqWorksForInvalidWalletHandle {

    NSError *ret;
    IndyHandle walletHandle = 0;

    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. get credentials
    NSString *proofRequest = @"{"
            "\"nonce\":\"123432421212\","
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","
            "\"requested_attributes\":{\"attr1_referent\":{\"name\":\"name\"}},"
            "\"requested_predicates\":{\"predicate1_referent\":{\"name\":\"age\",\"p_type\":\">=\",\"p_value\":18}}"
            "}";
    IndyHandle invalidWalletHandle = walletHandle + 1;
    ret = [[AnoncredsUtils sharedInstance] proverGetCredentialsForProofReq:proofRequest
                                                              walletHandle:invalidWalletHandle
                                                           credentialsJson:nil];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"AnoncredsUtils::proverGetCredentialsForProofReqWithWalletHandle returned wrong error code");

}

// MARK: - Prover create proof works

- (void)testProverCreateProofWorks {
    NSError *ret;
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

    NSString *proofRequest = @"{"
            "\"nonce\":\"123432421212\","
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","
            "\"requested_attributes\":{\"attr1_referent\":{\"name\":\"name\"}},"
            "\"requested_predicates\":{\"predicate1_referent\":{\"name\":\"age\",\"p_type\":\">=\",\"p_value\":18}}"
            "}";
    NSString *credentialsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetCredentialsForProofReq:proofRequest
                                                              walletHandle:walletHandle
                                                           credentialsJson:&credentialsJson];
    NSString *requestedCredentialsJson = [NSString stringWithFormat:@"{"\
                                     "\"self_attested_attributes\":{},"\
                                     "\"requested_attributes\":{"\
                                        "\"attr1_referent\":{\"cred_id\":\"%@\",\"revealed\":true}},"\
                                    "\"requested_predicates\":{"\
                                        "\"predicate1_referent\":{\"cred_id\":\"%@\"}"\
                                     "}}", [[AnoncredsUtils sharedInstance] credentialId1], [[AnoncredsUtils sharedInstance] credentialId1]];

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

- (void)testProverCreateProofWorksForUsingNotSatisfyCredential {
    NSError *ret;
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

    NSString *proofRequest = @"{"
            "\"nonce\":\"123432421212\","
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","
            "\"requested_attributes\":{\"attr1_referent\":{\"name\":\"status\"}},"
            "\"requested_predicates\":{}"
            "}";

    NSString *requestedCredentialsJson = [NSString stringWithFormat:@"{"\
                                     "\"self_attested_attributes\":{},"\
                                     "\"requested_attributes\":{"\
                                        "\"attr1_referent\":{\"cred_id\":\"%@\",\"revealed\":true}},"\
                                     "\"requested_predicates\":{}}", [[AnoncredsUtils sharedInstance] credentialId1]];

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
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::proverCreateProofWithWalletHandle returned wrong code");
}

- (void)testProverCreateProofWorksForInvalidWalletHandle {

    NSError *ret;
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
    NSString *proofRequest = @"{"
            "\"nonce\":\"123432421212\","
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","
            "\"requested_attributes\":{\"attr1_referent\":{\"name\":\"name\"}},"
            "\"requested_predicates\":{}"
            "}";

    NSString *credentialId = @"CredentialId1";

    NSString *requestedCredentialsJson = [NSString stringWithFormat:@"{"\
                                     "\"self_attested_attributes\":{},"\
                                     "\"requested_attributes\":{"\
                                        "\"attr1_referent\":{\"cred_id\":\"%@\",\"revealed\":true}},"\
                                     "\"requested_predicates\":{}}", credentialId];

    NSString *schemasJson = [NSString stringWithFormat:@"{\"%@\":%@}", [[AnoncredsUtils sharedInstance] getGvtSchemaId], [[AnoncredsUtils sharedInstance] getGvtSchemaJson]];
    NSString *credentialDefsJson = [NSString stringWithFormat:@"{\"%@\":%@}", [[AnoncredsUtils sharedInstance] getIssuer1GvtCredDefId], credentialDefJson];
    NSString *revocStatesJson = @"{}";

    // 3. create proof
    NSString *proofJson;
    IndyHandle invalidWalletHandle = walletHandle + 1;
    ret = [[AnoncredsUtils sharedInstance] proverCreateProofForRequest:proofRequest
                                              requestedCredentialsJSON:requestedCredentialsJson
                                                        masterSecretID:[TestUtils commonMasterSecretName]
                                                           schemasJSON:schemasJson
                                                    credentialDefsJSON:credentialDefsJson
                                                       revocStatesJSON:revocStatesJson
                                                          walletHandle:invalidWalletHandle
                                                             proofJson:&proofJson];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"AnoncredsUtils::proverCreateProofWithWalletHandle returned wrong code");
}

// MARK: - Verifier verify proof
- (void)testVerifierVerifyProofWorksForCorrectProof {
    NSError *ret;

    // 2. verify proof

    NSString *proofRequest = @"{"\
            "\"nonce\":\"123432421212\","\
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","
            "\"requested_attributes\":{"\
                "\"attr1_referent\":{"\
                    "\"name\":\"name\"}},"\
            "\"requested_predicates\":{"\
                "\"predicate1_referent\":{"\
                    "\"name\":\"age\","\
                    "\"p_type\":\">=\","\
                    "\"p_value\":18}"\
                "}"\
            "}";


    NSString *schemasJson = [NSString stringWithFormat:@"{\"%@\":%@}", [[AnoncredsUtils sharedInstance] getGvtSchemaId], [[AnoncredsUtils sharedInstance] getGvtSchemaJson]];

    NSString *credentialDefsJson = [NSString stringWithFormat:@"{\"%@\":%@}", [[AnoncredsUtils sharedInstance] getIssuer1GvtCredDefId], [[AnoncredsUtils sharedInstance] gvtCredDef]];

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
    NSError *ret;
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

    // 2. verify proof

    NSString *proofRequest = @"{"\
        "\"nonce\":\"123432421212\","\
        "\"name\":\"proof_req_1\","\
        "\"version\":\"0.1\","\
        "\"requested_attributes\":{"\
            "\"attr1_referent\":{"\
                "\"name\":\"sex\"}},"\
        "\"requested_predicates\":{"\
            "\"predicate1_referent\":{"\
                "\"name\":\"height\","\
                "\"p_type\":\">=\","\
                "\"p_value\":180}"\
        "}"\
    "}";

    NSString *schemasJson = [NSString stringWithFormat:@"{\"%@\":%@}", [[AnoncredsUtils sharedInstance] getGvtSchemaId], [[AnoncredsUtils sharedInstance] getGvtSchemaJson]];

    NSString *credentialDefsJson = [NSString stringWithFormat:@"{\"%@\":%@}", [[AnoncredsUtils sharedInstance] getIssuer1GvtCredDefId], [[AnoncredsUtils sharedInstance] gvtCredDef]];

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
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::verifierVerifyProof returned wrong error");
}

- (void)testVerifierVerifyProofWorksForWrongProof {

    NSError *ret;
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

    // 2. verify proof
    NSString *proofRequest = @"{"\
            "\"nonce\":\"123432421212\","\
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","
            "\"requested_attributes\":{"\
                "\"attr1_referent\":{"\
                    "\"name\":\"name\"}},"\
            "\"requested_predicates\":{"\
                "\"predicate1_referent\":{"\
                    "\"name\":\"age\","\
                    "\"p_type\":\">=\","\
                    "\"p_value\":18}}"\
            "}";


    NSString *schemasJson = [NSString stringWithFormat:@"{\"%@\":%@}", [[AnoncredsUtils sharedInstance] getGvtSchemaId], [[AnoncredsUtils sharedInstance] getGvtSchemaJson]];

    NSString *credentialDefsJson = [NSString stringWithFormat:@"{\"%@\":%@}", [[AnoncredsUtils sharedInstance] getIssuer1GvtCredDefId], [[AnoncredsUtils sharedInstance] gvtCredDef]];

    NSString *revocRegDefsJSON = @"{}";
    NSString *revocRegsJson = @"{}";

    NSString *proofJson = [[AnoncredsUtils sharedInstance] proofJSON];

    proofJson = [proofJson stringByReplacingOccurrencesOfString:@"3208989715" withString:@"1111111111"];

    BOOL isValid = false;
    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProofRequest:proofRequest
                                                            proofJSON:proofJson
                                                          schemasJSON:schemasJson
                                                   credentialDefsJSON:credentialDefsJson
                                                     revocRegDefsJSON:revocRegDefsJSON
                                                        revocRegsJSON:revocRegsJson
                                                              isValid:&isValid];
    XCTAssertFalse(isValid, @"isValid is true! Should be false.");
}


@end
