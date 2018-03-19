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

- (NSString *)getXyzSchemaJson;

- (NSString *)getIssuer1GvtCredDefId;

- (NSString *)getIssuer1XyzCredDefId;

- (NSString *)getIssuer2GvtCredDefId;

- (NSString *)getCredentialOfferJson:(NSString *)issuerDid
                           credDefId:(NSString *)credDefId;

- (NSString *)getGvtCredentialValuesJson;

- (NSString *)getXyzCredentialValuesJson;

- (NSString *)getGvt2CredentialValuesJson;

- (NSString *)credentialId1;

- (NSString *)credentialId2;

- (NSString *)getGvtCredentialDef;

- (NSError *)issuerCreateSchemaForIssuerDID:(NSString *)issuerDid
                                       name:(NSString *)name
                                    version:(NSString *)version
                                      attrs:(NSString *)attrs
                                   schemaId:(NSString **)schemaId
                                 schemaJson:(NSString **)schemaJson;

- (NSError *)issuerCreateCredentialDefinitionWithWalletHandle:(IndyHandle)walletHandle
                                                    issuerDid:(NSString *)issuerDid
                                                   schemaJson:(NSString *)schemaJson
                                                          tag:(NSString *)tag
                                                         type:(NSString *)type
                                                   configJson:(NSString *)configJson
                                              credentialDefId:(NSString **)credentialDefId
                                            credentialDefJson:(NSString **)credentialDefJson;

- (NSError *)issuerCreateAndStoreRevocRegForWithWalletHandle:(IndyHandle)walletHandle
                                                   issuerDid:(NSString *)issuerDID
                                                        type:(NSString *)type
                                                         tag:(NSString *)tag
                                                   credDefId:(NSString *)credDefId
                                                  configJSON:(NSString *)configJSON
                                             tailsWriterType:(NSString *)tailsWriterType
                                           tailsWriterConfig:(NSString *)tailsWriterConfig
                                                  revocRegId:(NSString **)revocRegId
                                             revocRegDefJson:(NSString **)revocRegDefJson
                                           revocRegEntryJson:(NSString **)revocRegEntryJson;

- (NSError *)issuerRevokeCredentialForWalletHandle:(IndyHandle)walletHandle
                                          RevRegId:(NSString *)revRegId
                                 tailsReaderHandle:(NSNumber *)tailsReaderHandle
                                    userRevocIndex:(NSNumber *)userRevocIndex
                                 revocRegDeltaJson:(NSString **)revocRegDeltaJson;

- (NSError *)issuerRecoverCredentialForWalletHandle:(IndyHandle)walletHandle
                                           RevRegId:(NSString *)revRegId
                                  tailsReaderHandle:(NSNumber *)tailsReaderHandle
                                     userRevocIndex:(NSNumber *)userRevocIndex
                                  revocRegDeltaJson:(NSString **)revocRegDeltaJson;

- (NSError *)issuerCreateCredentialOfferWithWalletHandle:(IndyHandle)walletHandle
                                         credentialDefId:(NSString *)credentialDefId
                                               issuerDid:(NSString *)issuerDid
                                               proverDid:(NSString *)proverDid
                                     credentialOfferJson:(NSString **)credentialOfferJson;

- (NSError *)issuerCreateCredentialWithWalletHandle:(IndyHandle)walletHandle
                                  credentialReqJson:(NSString *)credentialReqJson
                               credentialValuesJson:(NSString *)credentialValuesJson
                                           revRegId:(NSString *)revRegId
                                  tailsReaderHandle:(NSNumber *)tailsReaderHandle
                                     userRevocIndex:(NSNumber *)userRevocIndex
                                  outCredentialJson:(NSString **)xCredentialJson
                               outRevocRegDeltaJSON:(NSString **)revocRegDeltaJson;

- (NSError *)proverCreateMasterSecretNamed:(NSString *)masterSecretName
                              walletHandle:(IndyHandle)walletHandle;

- (NSError *)proverStoreCredentialOffer:(IndyHandle)walletHandle
                    credentialOfferJson:(NSString *)str;

- (NSError *)proverGetCredentialOffers:(IndyHandle)walletHandle
                            filterJson:(NSString *)filterJson
               outCredentialOffersJSON:(NSString **)outJson;

- (NSError *)proverCreateAndStoreCredentialReqWithDef:(NSString *)credentialDefJSON
                                            proverDid:(NSString *)proverDid
                                  credentialOfferJson:(NSString *)credentialOfferJSON
                                     masterSecretName:(NSString *)name
                                         walletHandle:(IndyHandle)walletHandle
                                 outCredentialReqJson:(NSString **)outJson;

- (NSError *)proverStoreCredentialWithWalletHandle:(IndyHandle)walletHandle
                                      credentialId:(NSString *)credentialId
                                   credentialsJson:(NSString *)credentialsJson
                                     revRegDefJSON:(NSString *)revRegDefJSON;

- (NSError *)proverGetCredentialsForProofReqWithWalletHandle:(IndyHandle)walletHandle
                                            proofRequestJson:(NSString *)proofRequestJson
                                          outCredentialsJson:(NSString **)outCredentialsJson;

- (NSError *)proverGetCredentialsForWalletHandle:(IndyHandle)walletHandle
                                      filterJson:(NSString *)filterJson
                               outCredentilsJson:(NSString **)credentialsJson;

- (NSError *)proverCreateProofWithWalletHandle:(IndyHandle)walletHandle
                                  proofReqJson:(NSString *)proofReqJson
                      requestedCredentialsJson:(NSString *)requestedCredentialsJson
                                   schemasJson:(NSString *)schemasJson
                              masterSecretName:(NSString *)masterSecreteName
                            credentialDefsJson:(NSString *)credentialDefsJson
                                revocInfosJSON:(NSString *)revocInfosJSON
                                  outProofJson:(NSString **)outProofJson;

- (NSError *)createRevocationInfoForTimestamp:(NSNumber *)timestamp
                                revRegDefJSON:(NSString *)revRegDefJSON
                              revRegDeltaJSON:(NSString *)revRegDeltaJSON
                            tailsReaderHandle:(NSNumber *)tailsReaderHandle
                                       revIdx:(NSNumber *)revIdx
                                  revInfoJson:(NSString **)revInfoJson;

- (NSError *)updateRevocationInfoForTimestamp:(NSNumber *)timestamp
                                  revInfoJSON:(NSString *)revInfoJSON
                                revRegDefJSON:(NSString *)revRegDefJSON
                              revRegDeltaJSON:(NSString *)revRegDeltaJSON
                            tailsReaderHandle:(NSNumber *)tailsReaderHandle
                                       revIdx:(NSNumber *)revIdx
                           updatedRevInfoJson:(NSString **)updatedRevInfoJson;

- (NSError *)storeRevocationInfoForWalletHandle:(IndyHandle)walletHandle
                                             id:(NSString *)id
                                    revInfoJSON:(NSString *)revInfoJSON;

- (NSError *)getRevocationInfoForWalletHandle:(IndyHandle)walletHandle
                                           id:(NSString *)id
                                    timestamp:(NSNumber *)timestamp
                                  revInfoJson:(NSString **)revInfoJson;

- (NSError *)verifierVerifyProof:(NSString *)proofRequestJson
                       proofJson:(NSString *)proofJson
                     schemasJson:(NSString *)schemasJson
              credentialDefsJson:(NSString *)credentialDefsJson
                revocRegDefsJSON:(NSString *)revocRegDefsJSON
                   revocRegsJson:(NSString *)revocRegsJson
                        outValid:(BOOL *)isValid;

- (NSError *)initializeCommonWalletAndReturnHandle:(IndyHandle *)walletHandle
                                 credentialDefJson:(NSString **)credentialDefJson
                               credentialOfferJson:(NSString **)credentialOfferJson
                                 credentialReqJson:(NSString **)credentialReqJson
                                    credentialJson:(NSString **)credentialfJson;

@end
