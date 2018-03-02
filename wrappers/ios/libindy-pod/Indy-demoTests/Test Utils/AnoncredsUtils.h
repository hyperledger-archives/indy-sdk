//
//  AnoncredsUtils.h
//  Indy-demo
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>

@interface AnoncredsUtils : XCTestCase

+ (AnoncredsUtils *)sharedInstance;

- (NSString *)getGvtSchemaKey;

- (NSString *)getGvtSchemaJson:(NSNumber *)seqNo;

- (NSString *)getSchemaJson:(NSString *)schemaName;

- (NSString *)getClaimOfferJson:(NSString *)issuerDid
                      schemaKey:(NSString *)schemaKey;

- (NSString *)getXyzSchemaKey;

- (NSString *)getGvtClaimJson;

- (NSString *)getXyzSchemaJson:(NSNumber *)schemaSeqNo;

- (NSString *)getXyzClaimJson;

- (NSString *)getGvtClaimDef;

- (NSString *)getGvtClaimRequest;

- (NSString *)getClaimDefIdForIssuerDid:(NSString *)issuerDid
                            schemaSeqNo:(NSNumber *)schemaSeqNo;


/**
 
 @param proofClaims Dictionary with format:
 {
 "requested_attr1_referent": [claim1, claim2],
 "requested_attr2_referent": [],
 "requested_attr3_referent": [claim3],
 "requested_predicate_1_referent": [claim1, claim3],
 }
 @return Array of unique claims
 */
- (NSArray *)getUniqueClaimsFrom:(NSDictionary *)proofClaims;


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

- (NSError *)issuerCreateClaimWithWalletHandle:(IndyHandle)walletHandle
                                  claimReqJson:(NSString *)claimReqJson
                                     claimJson:(NSString *)claimJson
                                userRevocIndex:(NSNumber *)userRevocIndex
                                  outClaimJson:(NSString **)xClaimJson
                         outRevocRegUpdateJSON:(NSString **)revocRegUpdateJSON;

- (NSError *)issuerCreateClaimDefinifionWithWalletHandle:(IndyHandle)walletHandle
                                               issuerDid:(NSString *)issuerDid
                                              schemaJson:(NSString *)schemaJson
                                           signatureType:(NSString *)signatureType
                                          createNonRevoc:(BOOL)createNonRevoc
                                            claimDefJson:(NSString **)claimDefJson;

- (NSError *)issuerCreateClaimOfferWithWalletHandle:(IndyHandle)walletHandle
                                         schemaJson:(NSString *)schemaJson
                                          issuerDid:(NSString *)issuerDid
                                          proverDid:(NSString *)proverDid
                                     claimOfferJson:(NSString **)claimOfferJson;

- (NSError *)proverStoreClaimWithWalletHandle:(IndyHandle)walletHandle
                                   claimsJson:(NSString *)str
                                   revRegJSON:(NSString *)revRegJSON;

- (NSError *)proverGetClaimsForProofReqWithWalletHandle:(IndyHandle)walletHandle
                                       proofRequestJson:(NSString *)str
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
                                 revocRegsJson:(NSString *)revocRegsJson
                                  outProofJson:(NSString **)outProofJson;

- (NSError *)verifierVerifyProof:(NSString *)proofRequestJson
                       proofJson:(NSString *)proofJson
                     schemasJson:(NSString *)schemasJson
                   claimDefsJson:(NSString *)claimDefsJson
                   revocRegsJson:(NSString *)revocRegsJson
                        outValid:(BOOL *)isValid;

- (NSError *)initializeCommonWalletAndReturnHandle:(IndyHandle *)walletHandle
                                      claimDefJson:(NSString **)claimDefJson
                                    claimOfferJson:(NSString **)claimOfferJson
                                      claimReqJson:(NSString **)claimReqJson
                                         claimJson:(NSString **)claimJson;

@end
