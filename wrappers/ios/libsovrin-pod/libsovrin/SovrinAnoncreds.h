//
//  SovrinAnoncreds.h
//  libsovrin
//


#import <Foundation/Foundation.h>
#import "SovrinTypes.h"

@interface SovrinAnoncreds : NSObject

+ (NSError*) issuerCreateAndStoreClaimDef:(SovrinHandle) walletHandle
                               schemaJSON:(NSString*) schema
                            signatureType:(NSString*) signatureType
                           createNonRevoc:(BOOL) createNonRevoc
                               completion:(void (^)(NSError* error, NSString* claimDefJSON, NSString* claimDefUUID)) handler;

+ (NSError*) issuerCreateAndStoreRevocReg:(SovrinHandle) walletHandle
                            claimDefSeqNo:(NSNumber*) seqNo
                              maxClaimNum:(NSNumber*) maxClaimNum
                               completion:(void (^)(NSError* error, NSString* revocRegJSON, NSString* revocRegUUID)) handler;

+ (NSError*) issuerCreateClaim:(SovrinHandle) walletHandle
                  claimReqJSON:(NSString*) reqJSON
                     claimJSON:(NSString*) claimJSON
                 revocRegSeqNo:(NSNumber*) seqNo       // TODO: check how to deal with option<>
                userRevocIndex:(NSNumber*) revocIndex  // TODO: check how to deal with option<>
                    completion:(void (^)(NSError* error, NSString* revocRegUpdateJSON, NSString* claimJSON)) handler;

+ (NSError*) issuerRevokeClaim:(SovrinHandle) walletHandle
                 claimDefSeqNo:(NSNumber*) claimSeqNo
                 revocRegSeqNo:(NSNumber*) revocSeqNo
                userRevocIndex:(NSNumber*) revocIndex
                    completion:(void (^)(NSError* error, NSString* revocRegUpdateJSON)) handler;

+ (NSError*) proverStoreClaimOffer:(SovrinHandle) walletHandle
                    claimOfferJSON:(NSString*) json
                        completion:(void (^)(NSError* error)) handler;

+ (NSError*) proverGetClaimOffers:(SovrinHandle) walletHandle
                       filterJSON:(NSString*) json
                       completion:(void (^)(NSError* error, NSString* claimOffersJSON)) handler;

+ (NSError*) proverCreateMasterSecret:(SovrinHandle) walletHandle
                     masterSecretName:(NSString*) name
                           completion:(void (^)(NSError* error)) handler;

+ (NSError*) proverCreateAndStoreClaimReq:(SovrinHandle) walletHandle
                                proverDid:(NSString*) prover
                           claimOfferJSON:(NSString*) offerJson
                         masterSecretName:(NSString*) name
                             claimDefJSON:(NSString*) claimJson
                               completion:(void (^)(NSError* error, NSString* claimReqJSON)) handler;

+ (NSError*) proverStoreClaim:(SovrinHandle) walletHandle
                   claimsJSON:(NSString*) claimsJson
                   completion:(void (^)(NSError* error)) handler;

+ (NSError*) proverGetClaims:(SovrinHandle) walletHandle
                  filterJSON:(NSString*) json
                  completion:(void (^)(NSError* error, NSString* claimsJSON)) handler;

+ (NSError*) proverGetClaimsForProofReq:(SovrinHandle) walletHandle
                           proofReqJSON:(NSString*) json
                             completion:(void (^)(NSError* error, NSString* claimsJSON)) handler;

+ (NSError*) proverCreateProof:(SovrinHandle) walletHandle
                  proofReqJSON:(NSString*) reqJSON
           requestedClaimsJSON:(NSString*) claimsJSON
                   schemasJSON:(NSString*) schemasJSON
              masterSecretName:(NSString*) name
                 claimDefsJSON:(NSString*) claimDefsJson
                 revocRegsJSON:(NSString*) revocJson
                    completion:(void (^)(NSError* error, NSString* proofJSON)) handler;


+ (NSError*) verifierVerifyProof:(NSString*) proofReqJSON
                       proofJSON:(NSString*) proofJSON
                     schemasJSON:(NSString*) schemasJSON
                   claimDefsJSON:(NSString*) claimDefsJSON
                   revocRegsJSON:(NSString*) revocJSON
                      completion:(void (^)(NSError* error, BOOL valid)) handler;

@end
