//
//  AnoncredsUtils.h
//  Indy-demo
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>

@interface AnoncredsUtils : XCTestCase

+ (AnoncredsUtils *)sharedInstance;

- (NSString *)defaultCredentialDefConfig;

- (NSString *)getGvtSchemaId;

- (NSString *)getXyzSchemaId;

- (NSString *)getGvtSchemaJson;

- (NSString *)getIssuer1GvtCredDefId;

- (NSString *)getGvtCredentialValuesJson;

- (NSString *)getXyzCredentialValuesJson;

- (NSString *)getGvt2CredentialValuesJson;

- (NSString *)credentialId1;

- (NSString *)credentialId2;

- (NSString *)getGvtCredentialDef;

- (NSString *)gvtCredDef;

- (NSString *)proofJSON;

- (NSError *)issuerCreateSchemaWithName:(NSString *)name
                                version:(NSString *)version
                                  attrs:(NSString *)attrs
                              issuerDID:(NSString *)issuerDID
                               schemaId:(NSString **)schemaId
                             schemaJson:(NSString **)schemaJson;

- (NSError *)issuerCreateAndStoreCredentialDefForSchema:(NSString *)schemaJSON
                                              issuerDID:(NSString *)issuerDID
                                                    tag:(NSString *)tag
                                                   type:(NSString *)type
                                             configJSON:(NSString *)configJSON
                                           walletHandle:(IndyHandle)walletHandle
                                              credDefId:(NSString **)credentialDefId
                                            credDefJson:(NSString **)credentialDefJson;

- (NSError *)issuerCreateAndStoreRevocRegForCredentialDefId:(NSString *)credDefID
                                                  issuerDID:(NSString *)issuerDID
                                                       type:(NSString *)type
                                                        tag:(NSString *)tag
                                                 configJSON:(NSString *)configJSON
                                          tailsWriterHandle:(IndyHandle)tailsWriterHandle
                                               walletHandle:(IndyHandle)walletHandle
                                                 revocRegId:(NSString **)revocRegId
                                            revocRegDefJson:(NSString **)revocRegDefJson
                                          revocRegEntryJson:(NSString **)revocRegEntryJson;

- (NSError *)issuerRevokeCredentialByCredRevocId:(NSString *)credRevocId
                                        revRegId:(NSString *)revRegId
                         blobStorageReaderHandle:(NSNumber *)blobStorageReaderHandle
                                    walletHandle:(IndyHandle)walletHandle
                               revocRegDeltaJson:(NSString **)revocRegDeltaJson;

- (NSError *)issuerRecoverCredentialByCredRevocId:(NSString *)credRevocId
                                         revRegId:(NSString *)revRegId
                          blobStorageReaderHandle:(NSNumber *)blobStorageReaderHandle
                                     walletHandle:(IndyHandle)walletHandle
                                revocRegDeltaJson:(NSString **)revocRegDeltaJson;

- (NSError *)issuerMergerRevocationRegistryDelta:(NSString *)revRegDelta
                                       withDelta:(NSString *)otherRevRegDelta
                               mergedRevRegDelta:(NSString **)mergedRevRegDelta;

- (NSError *)issuerCreateCredentialOfferForCredDefId:(NSString *)credDefID
                                        walletHandle:(IndyHandle)walletHandle
                                       credOfferJson:(NSString **)credOfferJson;

- (NSError *)issuerCreateCredentialForCredentialRequest:(NSString *)credReqJSON
                                          credOfferJSON:(NSString *)credOfferJSON
                                         credValuesJSON:(NSString *)credValuesJSON
                                               revRegId:(NSString *)revRegId
                                blobStorageReaderHandle:(NSNumber *)blobStorageReaderHandle
                                           walletHandle:(IndyHandle)walletHandle
                                               credJson:(NSString **)credJson
                                            credRevocId:(NSString **)credRevocId
                                      revocRegDeltaJSON:(NSString **)revocRegDeltaJson;

- (NSError *)proverCreateMasterSecret:(NSString *)masterSecretId
                         walletHandle:(IndyHandle)walletHandle
                    outMasterSecretId:(NSString **)outMasterSecretId;


- (NSError *)proverCreateCredentialReqForCredentialOffer:(NSString *)credOfferJSON
                                       credentialDefJSON:(NSString *)credentialDefJSON
                                               proverDID:(NSString *)proverDID
                                          masterSecretID:(NSString *)masterSecretID
                                            walletHandle:(IndyHandle)walletHandle
                                             credReqJson:(NSString **)credReqJson
                                     credReqMetadataJson:(NSString **)credReqMetadataJson;

- (NSError *)proverStoreCredential:(NSString *)credJson
                            credID:(NSString *)credID
                       credReqJSON:(NSString *)credReqJSON
               credReqMetadataJSON:(NSString *)credReqMetadataJSON
                       credDefJSON:(NSString *)credDefJSON
                     revRegDefJSON:(NSString *)revRegDefJSON
                      walletHandle:(IndyHandle)walletHandle
                         outCredId:(NSString **)outCredId;

- (NSError *)proverGetCredentialsForProofReq:(NSString *)proofReqJSON
                                walletHandle:(IndyHandle)walletHandle
                             credentialsJson:(NSString **)outCredentialsJson;

- (NSError *)proverGetCredentialsForFilter:(NSString *)filterJSON
                              walletHandle:(IndyHandle)walletHandle
                            credentilsJson:(NSString **)credentialsJson;

- (NSError *)proverCreateProofForRequest:(NSString *)proofRequestJSON
                requestedCredentialsJSON:(NSString *)requestedCredentialsJSON
                          masterSecretID:(NSString *)masterSecretID
                             schemasJSON:(NSString *)schemasJSON
                      credentialDefsJSON:(NSString *)credentialDefsJSON
                         revocStatesJSON:(NSString *)revocStatesJSON
                            walletHandle:(IndyHandle)walletHandle
                               proofJson:(NSString **)proofJson;

- (NSError *)createRevocationStateForCredRevID:(NSString *)credRevID
                                     timestamp:(NSNumber *)timestamp
                                 revRegDefJSON:(NSString *)revRegDefJSON
                               revRegDeltaJSON:(NSString *)revRegDeltaJSON
                       blobStorageReaderHandle:(NSNumber *)blobStorageReaderHandle
                                  revStateJson:(NSString **)revStateJson;

- (NSError *)updateRevocationState:(NSString *)revStateJSON
                         credRevID:(NSString *)credRevID
                         timestamp:(NSNumber *)timestamp
                     revRegDefJSON:(NSString *)revRegDefJSON
                   revRegDeltaJSON:(NSString *)revRegDeltaJSON
           blobStorageReaderHandle:(NSNumber *)blobStorageReaderHandle
               updatedRevStateJson:(NSString **)updatedRevStateJson;

- (NSError *)verifierVerifyProofRequest:(NSString *)proofRequestJson
                              proofJSON:(NSString *)proofJSON
                            schemasJSON:(NSString *)schemasJSON
                     credentialDefsJSON:(NSString *)credentialDefsJSON
                       revocRegDefsJSON:(NSString *)revocRegDefsJSON
                          revocRegsJSON:(NSString *)revocRegsJSON
                                isValid:(BOOL *)isValid;

- (NSError *)initializeCommonWalletAndReturnHandle:(IndyHandle *)walletHandle
                                 credentialDefJson:(NSString **)credentialDefJson
                               credentialOfferJson:(NSString **)credentialOfferJson
                                 credentialReqJson:(NSString **)credentialReqJson
                                    credentialJson:(NSString **)credentialfJson;

- (NSString *)toJson:(NSDictionary *)dictionary;

@end
