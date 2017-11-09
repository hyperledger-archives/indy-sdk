//
//  AnoncredsUtils.h
//  Indy-demo
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>

@interface AnoncredsUtils : XCTestCase

+ (AnoncredsUtils *)sharedInstance;

- (NSString *)getGvtSchemaJson:(NSNumber *)seqNo;

- (NSString *)getClaimOfferJson:(NSString *)issuerDid
                    schemaSeqNo:(NSNumber *)schemaSeqNo;

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
 "requested_attr1_uuid": [claim1, claim2],
 "requested_attr2_uuid": [],
 "requested_attr3_uuid": [claim3],
 "requested_predicate_1_uuid": [claim1, claim3],
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

- (NSError *)proverStoreClaimWithWalletHandle:(IndyHandle)walletHandle
                                   claimsJson:(NSString *)str;

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
                                      claimDefJson:(NSString **)claimDefJson;

@end
