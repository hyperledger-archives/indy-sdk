//
//  AnoncredsUtils.h
//  libsovrin-demo
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <libsovrin/libsovrin.h>

@interface AnoncredsUtils : XCTestCase

+ (AnoncredsUtils *)sharedInstance;

+ (NSString *)commonMasterSecretName;

+ (SovrinHandle)walletHandle;
+ (NSString *)claimDefJson;
+ (NSString *)issuerDid;

- (NSString *)getGvtSchemaJson:(NSNumber *)seqNo;

- (NSString *)getClaimOfferJson:(NSString *)issuerDid
                    schemaSeqNo:(NSNumber *)schemaSeqNo;

- (NSString *)getGvtClaimJson;
- (NSString *)getXyzSchemaJson:(NSNumber *)schemaSeqNo;
- (NSString *)getXyzClaimJson;

- (NSString *)getGvtClaimDef;
- (NSString *)getGvtClaimRequest;

- (NSError *) createClaimDefinitionAndSetLink:(SovrinHandle)walletHandle
                                       schema:(NSString *)schema
                                        seqNo:(NSNumber *)claimDefSeqNo
                                      outJson:(NSString **)outJson;


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


- (NSError *)proverCreateMasterSecret:(SovrinHandle)walletHandle
                     masterSecretName:(NSString *)name;

- (NSError *)proverStoreClaimOffer:(SovrinHandle)walletHandle
                    claimOfferJson:(NSString *)str;

- (NSError *)proverGetClaimOffers:(SovrinHandle)walletHandle
                       filterJson:(NSString *)filterJson
               outClaimOffersJSON:(NSString **)outJson;

- (NSError *)proverCreateAndStoreClaimReq:(SovrinHandle)walletHandle
                                proverDid:(NSString *)pd
                           claimOfferJson:(NSString *)coj
                             claimDefJson:(NSString *)cdj
                         masterSecretName:(NSString *)name
                          outClaimReqJson:(NSString **)outJson;

- (NSError *)issuerCreateClaimWithWalletHandle:(SovrinHandle)walletHandle
                                  claimReqJson:(NSString *)claimReqJson
                                     claimJson:(NSString *)claimJson
                                  outClaimJson:(NSString **)xClaimJson
                         outRevocRegUpdateJSON:(NSString **)revocRegUpdateJSON;

- (NSError *)issuerCreateClaimDefinifionWithWalletHandle:(SovrinHandle)walletHandle
                                               issuerDid:(NSString *)issuerDid
                                              schemaJson:(NSString *)schemaJson
                                           signatureType:(NSString *)signatureType
                                          createNonRevoc:(BOOL)createNonRevoc
                                            claimDefJson:(NSString **)claimDefJson
                                            claimDefUUID:(NSString **)claimDefUUID;

- (NSError *) proverStoreClaimWithWalletHandle:(SovrinHandle)walletHandle
                                    claimsJson:(NSString *)str;

- (NSError *)proverGetClaimsForProofReqWithWalletHandle:(SovrinHandle)walletHandle
                                       proofRequestJson:(NSString *)str
                                          outClaimsJson:(NSString **)outClaimsJson;

- (NSError *)proverGetClaimsForWalletHandle:(SovrinHandle)walletHandle
                                 filterJson:(NSString *)filterJson
                              outClaimsJson:(NSString **)claimsJson;

- (NSError *)proverCreateProofWithWalletHandle:(SovrinHandle)walletHandle
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

- (NSError *)initializeCommonWalletAndReturnHandle:(SovrinHandle *)walletHandle
                                      claimDefJson:(NSString **)claimDefJson;

@end
