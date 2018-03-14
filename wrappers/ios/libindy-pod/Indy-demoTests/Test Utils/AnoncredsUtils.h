//
//  AnoncredsUtils.h
//  Indy-demo
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>

@interface AnoncredsUtils : XCTestCase

+ (AnoncredsUtils *)sharedInstance;

- (NSString *)buildSchemaId:(NSString *)identifier
                       name:(NSString *)name
                    version:(NSString *)version;

- (NSString *)buildClaimDefId:(NSString *)identifier
                     schemaId:(NSString *)schemaId
                         type:(NSString *)type
                          tag:(NSString *)tag;

- (NSString *)defaultClaimDefConfig;

- (NSString *)getGvtSchemaId;

- (NSString *)getXyzSchemaId;

- (NSString *)getGvtSchemaJson;

- (NSString *)getXyzSchemaJson;

- (NSString *)getIssuer1GvtCredDefId;

- (NSString *)getIssuer1XyzCredDefId;

- (NSString *)getIssuer2GvtCredDefId;

- (NSString *)getClaimOfferJson:(NSString *)issuerDid
                      credDefId:(NSString *)credDefId;

- (NSString *)getGvtClaimValuesJson;

- (NSString *)getXyzClaimValuesJson;

- (NSString *)getGvt2ClaimValuesJson;

- (NSString *)getGvtClaimDef;

- (NSError *)issuerCreateClaimWithWalletHandle:(IndyHandle)walletHandle
                                  claimReqJson:(NSString *)claimReqJson
                               claimValuesJson:(NSString *)claimValuesJson
                                      revRegId:(NSString *)revRegId
                             tailsReaderHandle:(NSNumber *)tailsReaderHandle
                                userRevocIndex:(NSNumber *)userRevocIndex
                                  outClaimJson:(NSString **)xClaimJson
                          outRevocRegDeltaJSON:(NSString **)revocRegDeltaJson;

- (NSError *)issuerCreateSchemaForIssuerDID:(NSString *)issuerDid
                                       name:(NSString *)name
                                    version:(NSString *)version
                                      attrs:(NSString *)attrs
                                   schemaId:(NSString **)schemaId
                                 schemaJson:(NSString **)schemaJson;

- (NSError *)issuerCreateClaimDefinifionWithWalletHandle:(IndyHandle)walletHandle
                                               issuerDid:(NSString *)issuerDid
                                              schemaJson:(NSString *)schemaJson
                                                     tag:(NSString *)tag
                                                    type:(NSString *)type
                                              configJson:(NSString *)configJson
                                              claimDefId:(NSString **)claimDefId
                                            claimDefJson:(NSString **)claimDefJson;

- (NSError *)issuerCreateClaimOfferWithWalletHandle:(IndyHandle)walletHandle
                                         claimDefId:(NSString *)claimDefId
                                          issuerDid:(NSString *)issuerDid
                                          proverDid:(NSString *)proverDid
                                     claimOfferJson:(NSString **)claimOfferJson;

- (NSError *)proverCreateMasterSecretNamed:(NSString *)masterSecretName
                              walletHandle:(IndyHandle)walletHandle;

- (NSError *)proverStoreClaimOffer:(IndyHandle)walletHandle
                    claimOfferJson:(NSString *)str;

- (NSError *)proverGetClaimOffers:(IndyHandle)walletHandle
                       filterJson:(NSString *)filterJson
               outClaimOffersJSON:(NSString **)outJson;

- (NSError *)proverCreateAndStoreClaimReqWithDef:(NSString *)claimDefJSON
                                       proverDid:(NSString *)proverDid
                                  claimOfferJson:(NSString *)claimOfferJSON
                                masterSecretName:(NSString *)name
                                    walletHandle:(IndyHandle)walletHandle
                                 outClaimReqJson:(NSString **)outJson;

- (NSError *)proverStoreClaimWithWalletHandle:(IndyHandle)walletHandle
                                      claimId:(NSString *)claimId
                                   claimsJson:(NSString *)claimsJson
                                revRegDefJSON:(NSString *)revRegDefJSON;

- (NSError *)proverGetClaimsForProofReqWithWalletHandle:(IndyHandle)walletHandle
                                       proofRequestJson:(NSString *)proofRequestJson
                                          outClaimsJson:(NSString **)outClaimsJson;

- (NSError *)proverGetClaimsForWalletHandle:(IndyHandle)walletHandle
                                 filterJson:(NSString *)filterJson
                              outClaimsJson:(NSString **)claimsJson;

- (NSError *)proverCreateProofWithWalletHandle:(IndyHandle)walletHandle
                                  proofReqJson:(NSString *)proofReqJson
                           requestedClaimsJson:(NSString *)requestedClaimsJson
                                   schemasJson:(NSString *)schemasJson
                              masterSecretName:(NSString *)masterSecreteName
                                 claimDefsJson:(NSString *)claimDefsJson
                                revocInfosJSON:(NSString *)revocInfosJSON
                                  outProofJson:(NSString **)outProofJson;

- (NSError *)verifierVerifyProof:(NSString *)proofRequestJson
                       proofJson:(NSString *)proofJson
                     schemasJson:(NSString *)schemasJson
                   claimDefsJson:(NSString *)claimDefsJson
                revocRegDefsJSON:(NSString *)revocRegDefsJSON
                   revocRegsJson:(NSString *)revocRegsJson
                        outValid:(BOOL *)isValid;

- (NSError *)initializeCommonWalletAndReturnHandle:(IndyHandle *)walletHandle
                                      claimDefJson:(NSString **)claimDefJson
                                    claimOfferJson:(NSString **)claimOfferJson
                                      claimReqJson:(NSString **)claimReqJson
                                         claimJson:(NSString **)claimJson;

@end
