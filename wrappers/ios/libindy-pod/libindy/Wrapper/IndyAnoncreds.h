//
//  IndyAnoncreds.h
//  libindy
//


#import <Foundation/Foundation.h>
#import "IndyTypes.h"

@interface IndyAnoncreds : NSObject

+ (NSError *)issuerCreateAndStoreClaimDefWithWalletHandle:(IndyHandle)walletHandle
                                                issuerDid:(NSString *)issuerDid
                                               schemaJSON:(NSString *)schema
                                            signatureType:(NSString *)signatureType
                                           createNonRevoc:(BOOL)createNonRevoc
                                               completion:(void (^)(NSError *error, NSString *claimDefJSON)) handler;

+ (NSError *)issuerCreateAndStoreRevocRegWithWalletHandle:(IndyHandle)walletHandle
                                                issuerDid:(NSString *)issuerDid
                                            claimDefSeqNo:(NSNumber *)seqNo
                                              maxClaimNum:(NSNumber *)maxClaimNum
                                               completion:(void (^)(NSError *error, NSString *revocRegJSON, NSString *revocRegUUID)) handler;

+ (NSError *)issuerCreateClaimWithWalletHandle:(IndyHandle)walletHandle
                                  claimReqJSON:(NSString *)reqJSON
                                     claimJSON:(NSString *)claimJSON
                                userRevocIndex:(NSNumber *)revocIndex
                                    completion:(void (^)(NSError *error, NSString *revocRegUpdateJSON, NSString *claimJSON)) handler;

+ (NSError *)issuerRevokeClaimWithWalletHandle:(IndyHandle)walletHandle
                                     issuerDid:(NSString *)issuerDid
                                   schemaSeqNo:(NSNumber *)schemaSeqNo
                                userRevocIndex:(NSNumber *)revocIndex
                                    completion:(void (^)(NSError *error, NSString *revocRegUpdateJSON)) handler;

+ (NSError *)proverStoreClaimOfferWithWalletHandle:(IndyHandle)walletHandle
                                    claimOfferJSON:(NSString *)json
                                        completion:(void (^)(NSError *error)) handler;

+ (NSError *)proverGetClaimOffersWithWalletHandle:(IndyHandle)walletHandle
                                       filterJSON:(NSString *)json
                                       completion:(void (^)(NSError *error, NSString *claimOffersJSON)) handler;

+ (NSError *)proverCreateMasterSecretWithWalletHandle:(IndyHandle)walletHandle
                                     masterSecretName:(NSString *)name
                                           completion:(void (^)(NSError *error)) handler;

+ (NSError *)proverCreateAndStoreClaimReqWithWalletHandle:(IndyHandle)walletHandle
                                                proverDid:(NSString *)prover
                                           claimOfferJSON:(NSString *)offerJson
                                             claimDefJSON:(NSString *)claimJson
                                         masterSecretName:(NSString *)name
                                               completion:(void (^)(NSError *error, NSString *claimReqJSON)) handler;

+ (NSError *)proverStoreClaimWithWalletHandle:(IndyHandle)walletHandle
                                   claimsJSON:(NSString *)claimsJson
                                   completion:(void (^)(NSError *error)) handler;

+ (NSError *)proverGetClaimsWithWalletHandle:(IndyHandle)walletHandle
                                  filterJSON:(NSString *)json
                                  completion:(void (^)(NSError *error, NSString *claimsJSON)) handler;

+ (NSError *)proverGetClaimsForProofReqWithWalletHandle:(IndyHandle)walletHandle
                                           proofReqJSON:(NSString *)json
                                             completion:(void (^)(NSError *error, NSString *claimsJSON)) handler;

+ (NSError *)proverCreateProofWithWalletHandle:(IndyHandle)walletHandle
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
