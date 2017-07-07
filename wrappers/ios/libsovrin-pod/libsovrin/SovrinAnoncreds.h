//
//  SovrinAnoncreds.h
//  libsovrin
//


#import <Foundation/Foundation.h>
#import "SovrinTypes.h"

@interface SovrinAnoncreds : NSObject

+ (NSError *)issuerCreateAndStoreClaimDefWithWalletHandle:(SovrinHandle)walletHandle
                                                issuerDid:(NSString *)issuerDid
                                               schemaJSON:(NSString *)schema
                                            signatureType:(NSString *)signatureType
                                           createNonRevoc:(BOOL)createNonRevoc
                                               completion:(void (^)(NSError *error, NSString *claimDefJSON, NSString *claimDefUUID)) handler;

+ (NSError *)issuerCreateAndStoreRevocRegWithWalletHandle:(SovrinHandle)walletHandle
                                                issuerDid:(NSString *)issuerDid
                                            claimDefSeqNo:(NSNumber *)seqNo
                                              maxClaimNum:(NSNumber *)maxClaimNum
                                               completion:(void (^)(NSError *error, NSString *revocRegJSON, NSString *revocRegUUID)) handler;

+ (NSError *)issuerCreateClaimWithWalletHandle:(SovrinHandle)walletHandle
                                  claimReqJSON:(NSString *)reqJSON
                                     claimJSON:(NSString *)claimJSON
                                 revocRegSeqNo:(NSNumber *)seqNo       // TODO: check how to deal with option<>
                                userRevocIndex:(NSNumber *)revocIndex  // TODO: check how to deal with option<>
                                    completion:(void (^)(NSError *error, NSString *revocRegUpdateJSON, NSString *claimJSON)) handler;

+ (NSError *)issuerRevokeClaimWithWalletHandle:(SovrinHandle)walletHandle
                                 revocRegSeqNo:(NSNumber *)revocSeqNo
                                userRevocIndex:(NSNumber *)revocIndex
                                    completion:(void (^)(NSError *error, NSString *revocRegUpdateJSON)) handler;

+ (NSError *)proverStoreClaimOfferWithWalletHandle:(SovrinHandle)walletHandle
                                    claimOfferJSON:(NSString *)json
                                        completion:(void (^)(NSError *error)) handler;

+ (NSError *)proverGetClaimOffersWithWalletHandle:(SovrinHandle)walletHandle
                                       filterJSON:(NSString *)json
                                       completion:(void (^)(NSError *error, NSString *claimOffersJSON)) handler;

+ (NSError *)proverCreateMasterSecretWithWalletHandle:(SovrinHandle)walletHandle
                                     masterSecretName:(NSString *)name
                                           completion:(void (^)(NSError *error)) handler;

+ (NSError *)proverCreateAndStoreClaimReqWithWalletHandle:(SovrinHandle)walletHandle
                                                proverDid:(NSString *)prover
                                           claimOfferJSON:(NSString *)offerJson
                                             claimDefJSON:(NSString *)claimJson
                                         masterSecretName:(NSString *)name
                                               completion:(void (^)(NSError *error, NSString *claimReqJSON)) handler;

+ (NSError *)proverStoreClaimWithWalletHandle:(SovrinHandle)walletHandle
                                   claimsJSON:(NSString *)claimsJson
                                   completion:(void (^)(NSError *error)) handler;

+ (NSError *)proverGetClaimsWithWalletHandle:(SovrinHandle)walletHandle
                                  filterJSON:(NSString *)json
                                  completion:(void (^)(NSError *error, NSString *claimsJSON)) handler;

+ (NSError *)proverGetClaimsForProofReqWithWalletHandle:(SovrinHandle)walletHandle
                                           proofReqJSON:(NSString *)json
                                             completion:(void (^)(NSError *error, NSString *claimsJSON)) handler;

+ (NSError *)proverCreateProofWithWalletHandle:(SovrinHandle)walletHandle
                                  proofReqJSON:(NSString *)reqJSON
                           requestedClaimsJSON:(NSString *)claimsJSON
                                   schemasJSON:(NSString *)schemasJSON
                              masterSecretName:(NSString *)name
                                 claimDefsJSON:(NSString *)claimDefsJSON
                                 revocRegsJSON:(NSString *)revocJSON
                                    completion:(void (^)(NSError *error, NSString *proofJSON)) handler;

+ (NSError *)verifierVerifyProofWithWalletHandle:(NSString *)proofReqJSON
                                       proofJSON:(NSString *)proofJSON
                                     schemasJSON:(NSString *)schemasJSON
                                   claimDefsJSON:(NSString *)claimDefsJSON
                                   revocRegsJSON:(NSString *)revocJSON
                                      completion:(void (^)(NSError *error, BOOL valid)) handler;

@end
