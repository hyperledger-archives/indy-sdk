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
#import "BlobStorageUtils.h"

@interface AnoncredsLedgerInteraction : XCTestCase

@end

@implementation AnoncredsLedgerInteraction

- (void)setUp {
    [super setUp];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

- (void)testAnoncredsRevocationInteractionForIssuanceOnDemand {
    [TestUtils cleanupStorage];

    NSError *ret;

    // Create ledger config from genesis txn file
    NSString *poolName = @"pool1";
    NSString *txnFilePath = [[PoolUtils sharedInstance] createGenesisTxnFileForTestPool:poolName
                                                                             nodesCount:nil
                                                                            txnFilePath:nil];
    NSString *poolConfig = [[PoolUtils sharedInstance] poolConfigJsonForTxnFilePath:txnFilePath];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName() failed!");

    ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                              poolConfig:poolConfig];
    XCTAssertEqual(ret.code, Success, @"createPoolLedgerConfigWithPoolName() failed!");

    //  Open pool ledger
    __block IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName config:nil poolHandler:&poolHandle];

    // Create Issuer wallet, get wallet handle
    IndyHandle issuerWalletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&issuerWalletHandle];
    XCTAssertEqual(ret.code, Success, @"createAndOpenWalletWithPoolName() failed!");

    // Create Prover wallet, get wallet handle
    IndyHandle proverWalletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&proverWalletHandle];
    XCTAssertEqual(ret.code, Success, @"createAndOpenWalletWithPoolName() failed!");

    // Obtain default trustee did
    NSString *trusteeDid = nil;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:issuerWalletHandle
                                                                    seed:@"000000000000000000000000Trustee1"
                                                                outMyDid:&trusteeDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"createAndStoreMyDidWithWalletHandle() failed!");

    // Obtain issuer did
    NSString *issuerDid = nil;
    NSString *issuerVerkey = nil;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:issuerWalletHandle
                                                       myDidJson:@"{}"
                                                        outMyDid:&issuerDid
                                                     outMyVerkey:&issuerVerkey];
    XCTAssertEqual(ret.code, Success, @"createMyDidWithWalletHandle() failed!");

    // Trustee send Issuer DID to ledger
    NSString *nymRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:issuerDid
                                                                 verkey:issuerVerkey
                                                                  alias:nil
                                                                   role:@"TRUSTEE"
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"buildNymRequestWithSubmitterDid() failed!");

    NSString *nymResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:issuerWalletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest
                                                           outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, Success, @"signAndSubmitRequestWithPoolHandle() failed!");

    // Obtain prover did
    NSString *proverDid = nil;
    NSString *proverVerkey = nil;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:proverWalletHandle
                                                       myDidJson:@"{}"
                                                        outMyDid:&proverDid
                                                     outMyVerkey:&proverVerkey];
    XCTAssertEqual(ret.code, Success, @"createMyDidWithWalletHandle() failed!");

    // Issuer create Schema
    NSString *schemaId;
    NSString *schemaJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateSchemaWithName:[TestUtils gvtSchemaName]
                                                              version:[TestUtils schemaVersion]
                                                                attrs:[TestUtils gvtSchemaAttrs]
                                                            issuerDID:issuerDid
                                                             schemaId:&schemaId
                                                           schemaJson:&schemaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateSchemaWithName() failed!");

    // Issuer send schema to ledger
    NSString *schemaRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildSchemaRequestWithSubmitterDid:issuerDid
                                                                      data:schemaJson
                                                                resultJson:&schemaRequest];
    XCTAssertEqual(ret.code, Success, @"buildSchemaRequestWithSubmitterDid() failed!");

    NSString *schemaResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:issuerWalletHandle
                                                              submitterDid:issuerDid
                                                               requestJson:schemaRequest
                                                           outResponseJson:&schemaResponse];
    XCTAssertEqual(ret.code, Success, @"signAndSubmitRequestWithPoolHandle() failed!");

    // Issuer gets schema from ledger
    NSString *getSchemaRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildGetSchemaRequestWithSubmitterDid:issuerDid
                                                                           id:schemaId
                                                                   resultJson:&getSchemaRequest];
    XCTAssertEqual(ret.code, Success, @"buildGetSchemaRequestWithSubmitterDid() failed!");

    NSString *getSchemaResponse = nil;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:getSchemaRequest
                                                       response:&getSchemaResponse];
    XCTAssertEqual(ret.code, Success, @"sendRequestWithPoolHandle() failed!");

    ret = [[LedgerUtils sharedInstance] parseGetSchemaResponse:getSchemaResponse
                                                      schemaId:&schemaId
                                                    schemaJson:&schemaJson];
    XCTAssertEqual(ret.code, Success, @"parseGetSchemaResponse() failed!");

    // Issuer create credential definition
    NSString *credentialDefId;
    NSString *credentialDefJSON;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateAndStoreCredentialDefForSchema:schemaJson
                                                                            issuerDID:issuerDid
                                                                                  tag:[TestUtils tag]
                                                                                 type:nil
                                                                           configJSON:@"{\"support_revocation\": true}"
                                                                         walletHandle:issuerWalletHandle
                                                                            credDefId:&credentialDefId
                                                                          credDefJson:&credentialDefJSON];
    XCTAssertEqual(ret.code, Success, @"issuerCreateAndStoreCredentialDefForSchema() failed!");

    // Issuer send credential definition to ledger
    NSString *credDefRequestJson;
    ret = [[LedgerUtils sharedInstance] buildCredDefRequestWithSubmitterDid:issuerDid
                                                                       data:credentialDefJSON
                                                                 resultJson:&credDefRequestJson];
    XCTAssertEqual(ret.code, Success, @"buildCredDefRequestWithSubmitterDid() failed!");

    NSString *credDefResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:issuerWalletHandle
                                                              submitterDid:issuerDid
                                                               requestJson:credDefRequestJson
                                                           outResponseJson:&credDefResponse];
    XCTAssertEqual(ret.code, Success, @"signAndSubmitRequestWithPoolHandle() failed!");

    // Issuer create revocation registry
    NSString *configJson = @"{\"max_cred_num\":5, \"issuance_type\":\"ISSUANCE_ON_DEMAND\"}";
    NSString *tailsWriterConfig = [NSString stringWithFormat:@"{\"base_dir\":\"%@\", \"uri_pattern\":\"\"}", [TestUtils tmpFilePathAppending:@"tails"]];

    NSNumber *tailsWriterHandle = nil;
    ret = [[BlobStorageUtils sharedInstance] openWriterWithType:[TestUtils defaultType]
                                                         config:tailsWriterConfig
                                                         handle:&tailsWriterHandle];
    XCTAssertEqual(ret.code, Success, @"openWriterWithType() failed!");

    NSString *revocRegId;
    NSString *revocRegDefJson;
    NSString *revocRegEntryJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateAndStoreRevocRegForCredentialDefId:credentialDefId
                                                                                issuerDID:issuerDid
                                                                                     type:nil
                                                                                      tag:[TestUtils tag]
                                                                               configJSON:configJson
                                                                        tailsWriterHandle:[tailsWriterHandle intValue]
                                                                             walletHandle:issuerWalletHandle
                                                                               revocRegId:&revocRegId
                                                                          revocRegDefJson:&revocRegDefJson
                                                                        revocRegEntryJson:&revocRegEntryJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateAndStoreRevocRegForCredentialDefId() failed!");

    // Issuer send revocation registry definition to ledger
    NSString *revocRegDefRequestJson;
    ret = [[LedgerUtils sharedInstance] buildRevocRegDefRequestWithSubmitterDid:issuerDid
                                                                           data:revocRegDefJson
                                                                     resultJson:&revocRegDefRequestJson];
    XCTAssertEqual(ret.code, Success, @"buildRevocRegDefRequestWithSubmitterDid() failed!");

    NSString *revocRegDefResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:issuerWalletHandle
                                                              submitterDid:issuerDid
                                                               requestJson:revocRegDefRequestJson
                                                           outResponseJson:&revocRegDefResponse];
    XCTAssertEqual(ret.code, Success, @"signAndSubmitRequestWithPoolHandle() failed!");

    // Issuer send revocation registry entry to ledger
    NSString *revocRegEntryRequestJson;
    ret = [[LedgerUtils sharedInstance] buildRevocRegEntryRequestWithSubmitterDid:issuerDid
                                                                             type:@"CL_ACCUM"
                                                                    revocRegDefId:revocRegId
                                                                            value:revocRegEntryJson
                                                                       resultJson:&revocRegEntryRequestJson];
    XCTAssertEqual(ret.code, Success, @"buildRevocRegEntrtyRequestWithSubmitterDid() failed!");

    NSString *revocRegEntryResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:issuerWalletHandle
                                                              submitterDid:issuerDid
                                                               requestJson:revocRegEntryRequestJson
                                                           outResponseJson:&revocRegEntryResponse];
    XCTAssertEqual(ret.code, Success, @"signAndSubmitRequestWithPoolHandle() failed!");

    // Issuance Credential for Prover

    // Prover create Master Secret
    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecret:[TestUtils commonMasterSecretName]
                                                       walletHandle:proverWalletHandle
                                                  outMasterSecretId:nil];
    XCTAssertEqual(ret.code, Success, @"proverCreateMasterSecret() failed!");

    // Issuer create Credential Offer
    NSString *credentialOfferJson = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateCredentialOfferForCredDefId:credentialDefId
                                                                      walletHandle:issuerWalletHandle
                                                                     credOfferJson:&credentialOfferJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateCredentialOfferForCredDefId() failed!");

    // Prover gets credential definition from ledger
    NSDictionary *credOffer = [NSDictionary fromString:credentialOfferJson];

    NSString *getCredDefRequest;
    ret = [[LedgerUtils sharedInstance] buildGetCredDefRequestWithSubmitterDid:proverDid
                                                                            id:credOffer[@"cred_def_id"]
                                                                    resultJson:&getCredDefRequest];
    XCTAssertEqual(ret.code, Success, @"buildGetCredDefRequestWithSubmitterDid() failed!");

    NSString *getCredDefResponse;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:getCredDefRequest
                                                       response:&getCredDefResponse];
    XCTAssertEqual(ret.code, Success, @"sendRequestWithPoolHandle() failed!");

    ret = [[LedgerUtils sharedInstance] parseGetCredDefResponse:getCredDefResponse
                                                      credDefId:&credentialDefId
                                                    credDefJson:&credentialDefJSON];
    XCTAssertEqual(ret.code, Success, @"parseGetCredDefResponse() failed!");

    // Prover create Credential Request
    NSString *credentialReq = nil;
    NSString *credentialReqMetadata = nil;
    ret = [[AnoncredsUtils sharedInstance] proverCreateCredentialReqForCredentialOffer:credentialOfferJson
                                                                     credentialDefJSON:credentialDefJSON
                                                                             proverDID:proverDid
                                                                        masterSecretID:[TestUtils commonMasterSecretName]
                                                                          walletHandle:proverWalletHandle
                                                                           credReqJson:&credentialReq
                                                                   credReqMetadataJson:&credentialReqMetadata];
    XCTAssertEqual(ret.code, Success, @"proverCreateCredentialReqForCredentialOffer() failed!");

    // Issuer create Tails reader
    NSNumber *blobStorageReaderHandle = nil;
    ret = [[BlobStorageUtils sharedInstance] openReaderWithType:[TestUtils defaultType]
                                                         config:tailsWriterConfig
                                                         handle:&blobStorageReaderHandle];
    XCTAssertEqual(ret.code, Success, @"openReaderWithType() failed!");

    // Issuer create Credential
    NSString *credentialJson = nil;
    NSString *credentialRevId = nil;
    NSString *revocRegDeltaJson = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateCredentialForCredentialRequest:credentialReq
                                                                        credOfferJSON:credentialOfferJson
                                                                       credValuesJSON:[[AnoncredsUtils sharedInstance] getGvtCredentialValuesJson]
                                                                             revRegId:revocRegId
                                                              blobStorageReaderHandle:blobStorageReaderHandle
                                                                         walletHandle:issuerWalletHandle
                                                                             credJson:&credentialJson
                                                                          credRevocId:&credentialRevId
                                                                    revocRegDeltaJSON:&revocRegDeltaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateCredentialForCredentialRequest() failed!");

    // Issuer send revocation registry delta to ledger
    ret = [[LedgerUtils sharedInstance] buildRevocRegEntryRequestWithSubmitterDid:issuerDid
                                                                             type:@"CL_ACCUM"
                                                                    revocRegDefId:revocRegId
                                                                            value:revocRegDeltaJson
                                                                       resultJson:&revocRegEntryRequestJson];
    XCTAssertEqual(ret.code, Success, @"buildRevocRegEntrtyRequestWithSubmitterDid() failed!");

    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:issuerWalletHandle
                                                              submitterDid:issuerDid
                                                               requestJson:revocRegEntryRequestJson
                                                           outResponseJson:&revocRegEntryResponse];
    XCTAssertEqual(ret.code, Success, @"signAndSubmitRequestWithPoolHandle() failed!");
    NSDictionary *revocRegEntry = [NSDictionary fromString:revocRegEntryResponse];
    NSNumber *entryTxnTime = revocRegEntry[@"result"][@"txnTime"];

    // Prover gets revocation registry definition from ledger
    NSDictionary *credential = [NSDictionary fromString:credentialJson];
    NSString *getRevocRegDefRequest;
    ret = [[LedgerUtils sharedInstance] buildGetRevocRegDefRequestWithSubmitterDid:proverDid
                                                                                id:credential[@"rev_reg_id"]
                                                                        resultJson:&getRevocRegDefRequest];
    XCTAssertEqual(ret.code, Success, @"buildGetRevocRegDefRequestWithSubmitterDid() failed!");

    NSString *getRevocRegDefResponse;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:getRevocRegDefRequest
                                                       response:&getRevocRegDefResponse];
    XCTAssertEqual(ret.code, Success, @"sendRequestWithPoolHandle() failed!");

    ret = [[LedgerUtils sharedInstance] parseGetRevocRegDefResponse:getRevocRegDefResponse
                                                      revocRegDefId:&revocRegId
                                                    revocRegDefJson:&revocRegDefJson];
    XCTAssertEqual(ret.code, Success, @"parseGetRevocRegDefResponse() failed!");

    // Prover store received Credential
    ret = [[AnoncredsUtils sharedInstance] proverStoreCredential:credentialJson
                                                          credID:[[AnoncredsUtils sharedInstance] credentialId1]
                                                     credReqJSON:credentialReq
                                             credReqMetadataJSON:credentialReqMetadata
                                                     credDefJSON:credentialDefJSON
                                                   revRegDefJSON:revocRegDefJson
                                                    walletHandle:proverWalletHandle
                                                       outCredId:nil];
    XCTAssertEqual(ret.code, Success, @"proverStoreCredential() failed!");

    // Prover gets Credentials for Proof Request
    NSString *proofReqJson = @"{"\
                             " \"nonce\":\"123432421212\","\
                             " \"name\":\"proof_req_1\","\
                             " \"version\":\"0.1\","\
                             " \"requested_attributes\":"\
                             "             {\"attr1_referent\":"\
                             "                        {"\
                             "                          \"name\":\"name\""\
                             "                        }"
            "             },"\
                             " \"requested_predicates\":"\
                             "             {"\
                             "              \"predicate1_referent\":"\
                             "                      {\"name\":\"age\",\"p_type\":\">=\",\"p_value\":18}"\
                             "             }"\
                             "}";

    NSString *credentialsJson = nil;
    ret = [[AnoncredsUtils sharedInstance] proverGetCredentialsForProofReq:proofReqJson
                                                              walletHandle:proverWalletHandle
                                                           credentialsJson:&credentialsJson];
    XCTAssertEqual(ret.code, Success, @"proverGetCredentialsForProofReq() failed!");

    NSDictionary *credentials = [NSDictionary fromString:credentialsJson];
    XCTAssertTrue(credentials, @"serialization failed");

    NSDictionary *credentials_for_attr_1 = credentials[@"attrs"][@"attr1_referent"][0];
    NSString *credentialReferent = credentials_for_attr_1[@"cred_info"][@"referent"];

    // Prover gets revocation registry delta from ledger
    NSNumber *timestamp = @([entryTxnTime intValue] + 100 );

    NSString *getRevocRegDeltaRequest;
    ret = [[LedgerUtils sharedInstance] buildGetRevocRegDeltaRequestWithSubmitterDid:proverDid
                                                                       revocRegDefId:credentials_for_attr_1[@"cred_info"][@"rev_reg_id"]
                                                                                from:@(-1)
                                                                                  to:timestamp
                                                                          resultJson:&getRevocRegDeltaRequest];
    XCTAssertEqual(ret.code, Success, @"buildGetRevocRegDeltaRequestWithSubmitterDid() failed!");

    NSString *getRevocRegDeltaResponse;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:getRevocRegDeltaRequest
                                                       response:&getRevocRegDeltaResponse];
    XCTAssertEqual(ret.code, Success, @"sendRequestWithPoolHandle() failed!");

    ret = [[LedgerUtils sharedInstance] parseGetRevocRegDeltaResponse:getRevocRegDeltaResponse
                                                        revocRegDefId:&revocRegId
                                                    revocRegDeltaJson:&revocRegDeltaJson
                                                            timestamp:&timestamp];
    XCTAssertEqual(ret.code, Success, @"parseGetRevocRegDeltaResponse() failed!");

    // Prover create Revocation State
    NSString *revocStateJson = nil;
    ret = [[AnoncredsUtils sharedInstance] createRevocationStateForCredRevID:credentialRevId
                                                                   timestamp:timestamp
                                                               revRegDefJSON:revocRegDefJson
                                                             revRegDeltaJSON:revocRegDeltaJson
                                                     blobStorageReaderHandle:blobStorageReaderHandle
                                                                revStateJson:&revocStateJson];
    XCTAssertEqual(ret.code, Success, @"createRevocationStateForCredRevID() failed!");

    // Prover create Proof
    NSString *requestedCredentialsJson = [NSString stringWithFormat:@"{\
                                     \"self_attested_attributes\":{},\
                                     \"requested_attributes\":{\"attr1_referent\":{\"cred_id\":\"%@\",\"revealed\":true, \"timestamp\":%@}},\
                                     \"requested_predicates\":{\"predicate1_referent\":{\"cred_id\":\"%@\", \"timestamp\":%@}}\
                                     }", credentialReferent, timestamp, credentialReferent, timestamp];


    NSString *schemasJson = [[AnoncredsUtils sharedInstance] toJson:@{schemaId: [NSDictionary fromString:schemaJson]}];

    NSString *credentialDefsJson = [[AnoncredsUtils sharedInstance] toJson:@{credentialDefId: [NSDictionary fromString:credentialDefJSON]}];

    NSString *revocStatesJson = [[AnoncredsUtils sharedInstance] toJson:@{revocRegId: @{[timestamp stringValue]: [NSDictionary fromString:revocStateJson]}}];

    NSString *proofJson;
    ret = [[AnoncredsUtils sharedInstance] proverCreateProofForRequest:proofReqJson
                                              requestedCredentialsJSON:requestedCredentialsJson
                                                        masterSecretID:[TestUtils commonMasterSecretName]
                                                           schemasJSON:schemasJson
                                                    credentialDefsJSON:credentialDefsJson
                                                       revocStatesJSON:revocStatesJson
                                                          walletHandle:proverWalletHandle
                                                             proofJson:&proofJson];
    XCTAssertEqual(ret.code, Success, @"proverCreateProofForRequest() failed!");

    NSDictionary *proof = [NSDictionary fromString:proofJson];
    NSDictionary *revealedAttr1 = proof[@"requested_proof"][@"revealed_attrs"][@"attr1_referent"];
    NSString *raw = revealedAttr1[@"raw"];
    XCTAssertTrue([raw isEqualToString:@"Alex"]);

    // Verifier gets revocation registry from ledger
    NSString *getRevocRegRequest;
    ret = [[LedgerUtils sharedInstance] buildGetRevocRegRequestWithSubmitterDid:[TestUtils proverDid]
                                                                  revocRegDefId:proof[@"identifiers"][0][@"rev_reg_id"]
                                                                      timestamp:proof[@"identifiers"][0][@"timestamp"]
                                                                     resultJson:&getRevocRegRequest];
    XCTAssertEqual(ret.code, Success, @"buildGetRevocRegRequestWithSubmitterDid() failed!");

    NSString *getRevocRegResponse;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:getRevocRegRequest
                                                       response:&getRevocRegResponse];
    XCTAssertEqual(ret.code, Success, @"sendRequestWithPoolHandle() failed!");

    NSString *revocRegJson = nil;
    ret = [[LedgerUtils sharedInstance] parseGetRevocRegResponse:getRevocRegResponse
                                                   revocRegDefId:&revocRegId
                                                    revocRegJson:&revocRegJson
                                                       timestamp:&timestamp];
    XCTAssertEqual(ret.code, Success, @"parseGetRevocRegResponse() failed!");

    NSString *revocRegDefsJson = [[AnoncredsUtils sharedInstance] toJson:@{revocRegId: [NSDictionary fromString:revocRegDefJson]}];

    NSString *revocRegsJson = [[AnoncredsUtils sharedInstance] toJson:@{revocRegId: @{[timestamp stringValue]: [NSDictionary fromString:revocRegJson]}}];

    // Verifier verify proof
    BOOL isValid = NO;

    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProofRequest:proofReqJson
                                                            proofJSON:proofJson
                                                          schemasJSON:schemasJson
                                                   credentialDefsJSON:credentialDefsJson
                                                     revocRegDefsJSON:revocRegDefsJson
                                                        revocRegsJSON:revocRegsJson
                                                              isValid:&isValid];
    XCTAssertEqual(ret.code, Success, @"verifierVerifyProofRequest() failed!");

    XCTAssertTrue(isValid, @"isValid == NO");

    // Issuer revokes credential
    ret = [[AnoncredsUtils sharedInstance] issuerRevokeCredentialByCredRevocId:credentialRevId
                                                                      revRegId:revocRegId
                                                       blobStorageReaderHandle:blobStorageReaderHandle
                                                                  walletHandle:issuerWalletHandle
                                                             revocRegDeltaJson:&revocRegDeltaJson];
    XCTAssertEqual(ret.code, Success, @"issuerRevokeCredentialByCredRevocId() failed!");

    // Issuer send revocation registry delta to ledger
    ret = [[LedgerUtils sharedInstance] buildRevocRegEntryRequestWithSubmitterDid:issuerDid
                                                                             type:@"CL_ACCUM"
                                                                    revocRegDefId:revocRegId
                                                                            value:revocRegDeltaJson
                                                                       resultJson:&revocRegEntryRequestJson];
    XCTAssertEqual(ret.code, Success, @"buildRevocRegEntrtyRequestWithSubmitterDid() failed!");

    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:issuerWalletHandle
                                                              submitterDid:issuerDid
                                                               requestJson:revocRegEntryRequestJson
                                                           outResponseJson:&revocRegEntryResponse];
    XCTAssertEqual(ret.code, Success, @"signAndSubmitRequestWithPoolHandle() failed!");

    // Prover gets revocation registry delta from ledger
    NSNumber *from = timestamp;
    NSNumber *to = @([from intValue] + 200);

    [NSThread sleepForTimeInterval:3];

    ret = [[LedgerUtils sharedInstance] buildGetRevocRegDeltaRequestWithSubmitterDid:proverDid
                                                                       revocRegDefId:credentials_for_attr_1[@"cred_info"][@"rev_reg_id"]
                                                                                from:from
                                                                                  to:to
                                                                          resultJson:&getRevocRegDeltaRequest];
    XCTAssertEqual(ret.code, Success, @"buildGetRevocRegDeltaRequestWithSubmitterDid() failed!");

    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:getRevocRegDeltaRequest
                                                       response:&getRevocRegDeltaResponse];
    XCTAssertEqual(ret.code, Success, @"sendRequestWithPoolHandle() failed!");

    ret = [[LedgerUtils sharedInstance] parseGetRevocRegDeltaResponse:getRevocRegDeltaResponse
                                                        revocRegDefId:&revocRegId
                                                    revocRegDeltaJson:&revocRegDeltaJson
                                                            timestamp:&timestamp];
    XCTAssertEqual(ret.code, Success, @"parseGetRevocRegDeltaResponse() failed!");

    // Prover create Revocation State
    ret = [[AnoncredsUtils sharedInstance] updateRevocationState:revocStateJson
                                                       credRevID:credentialRevId
                                                       timestamp:timestamp
                                                   revRegDefJSON:revocRegDefJson
                                                 revRegDeltaJSON:revocRegDeltaJson
                                         blobStorageReaderHandle:blobStorageReaderHandle
                                             updatedRevStateJson:&revocStateJson];
    XCTAssertEqual(ret.code, Success, @"createRevocationStateForCredRevID() failed!");

    // Prover create Proof
    requestedCredentialsJson = [NSString stringWithFormat:@"{\
                                     \"self_attested_attributes\":{},\
                                     \"requested_attributes\":{\"attr1_referent\":{\"cred_id\":\"%@\",\"revealed\":true, \"timestamp\":%@}},\
                                     \"requested_predicates\":{\"predicate1_referent\":{\"cred_id\":\"%@\", \"timestamp\":%@}}\
                                     }", credentialReferent, timestamp, credentialReferent, timestamp];


    revocStatesJson = [[AnoncredsUtils sharedInstance] toJson:@{revocRegId: @{[timestamp stringValue]: [NSDictionary fromString:revocStateJson]}}];

    ret = [[AnoncredsUtils sharedInstance] proverCreateProofForRequest:proofReqJson
                                              requestedCredentialsJSON:requestedCredentialsJson
                                                        masterSecretID:[TestUtils commonMasterSecretName]
                                                           schemasJSON:schemasJson
                                                    credentialDefsJSON:credentialDefsJson
                                                       revocStatesJSON:revocStatesJson
                                                          walletHandle:proverWalletHandle
                                                             proofJson:&proofJson];
    XCTAssertEqual(ret.code, Success, @"proverCreateProofForRequest() failed!");

    proof = [NSDictionary fromString:proofJson];

    // Verifier gets revocation registry from ledger
    ret = [[LedgerUtils sharedInstance] buildGetRevocRegRequestWithSubmitterDid:[TestUtils proverDid]
                                                                  revocRegDefId:proof[@"identifiers"][0][@"rev_reg_id"]
                                                                      timestamp:proof[@"identifiers"][0][@"timestamp"]
                                                                     resultJson:&getRevocRegRequest];
    XCTAssertEqual(ret.code, Success, @"buildGetRevocRegRequestWithSubmitterDid() failed!");

    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:getRevocRegRequest
                                                       response:&getRevocRegResponse];
    XCTAssertEqual(ret.code, Success, @"sendRequestWithPoolHandle() failed!");

    ret = [[LedgerUtils sharedInstance] parseGetRevocRegResponse:getRevocRegResponse
                                                   revocRegDefId:&revocRegId
                                                    revocRegJson:&revocRegJson
                                                       timestamp:&timestamp];
    XCTAssertEqual(ret.code, Success, @"parseGetRevocRegResponse() failed!");

    revocRegDefsJson = [[AnoncredsUtils sharedInstance] toJson:@{revocRegId: [NSDictionary fromString:revocRegDefJson]}];

    revocRegsJson = [[AnoncredsUtils sharedInstance] toJson:@{revocRegId: @{[timestamp stringValue]: [NSDictionary fromString:revocRegJson]}}];

    // Verifier verify proof
    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProofRequest:proofReqJson
                                                            proofJSON:proofJson
                                                          schemasJSON:schemasJson
                                                   credentialDefsJSON:credentialDefsJson
                                                     revocRegDefsJSON:revocRegDefsJson
                                                        revocRegsJSON:revocRegsJson
                                                              isValid:&isValid];
    XCTAssertEqual(ret.code, Success, @"verifierVerifyProofRequest() failed!");

    XCTAssertFalse(isValid, @"isValid == YES");

    ret = [[WalletUtils sharedInstance] closeWalletWithHandle:issuerWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:closeWalletWithHandle failed");

    ret = [[WalletUtils sharedInstance] closeWalletWithHandle:proverWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:closeWalletWithHandle failed");

    ret = [[PoolUtils sharedInstance] closeHandle:poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::closeHandle() failed!");

    [TestUtils cleanupStorage];
}

@end

