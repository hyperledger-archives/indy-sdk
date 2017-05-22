//
//  SovrinAnoncreds.m
//  libsovrin
//

#import "SovrinAnoncreds.h"
#import "SovrinCallbacks.h"
#import "sovrin_core.h"
#import "NSError+SovrinError.h"
#import "SovrinTypes.h"

@implementation SovrinAnoncreds

+ (NSError*) issuerCreateAndStoreClaimDef:(SovrinHandle) walletHandle
                               schemaJSON:(NSString*) schema
                            signatureType:(NSString*) signatureType
                           createNonRevoc:(BOOL) createNonRevoc
                               completion:(void (^)(NSError* error, NSString* claimDefJSON, NSString* claimDefUUID)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_issuer_create_and_store_claim_def(handle,
                                                   walletHandle,
                                                   [schema UTF8String],
                                                   [signatureType UTF8String],
                                                   (sovrin_bool_t) createNonRevoc,
                                                   SovrinWrapperCommon4PCallback
                                                  );
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) issuerCreateAndStoreRevocReg:(SovrinHandle) walletHandle
                            claimDefSeqNo:(NSNumber*) seqNo
                              maxClaimNum:(NSNumber*) maxClaimNum
                               completion:(void (^)(NSError* error, NSString* revocRegJSON, NSString* revocRegUUID)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_issuer_create_and_store_revoc_reg(handle,
                                                   walletHandle,
                                                   [seqNo intValue],
                                                   [maxClaimNum intValue],
                                                   SovrinWrapperCommon4PCallback
                                                  );
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) issuerCreateClaim:(SovrinHandle) walletHandle
                  claimReqJSON:(NSString*) reqJSON
                     claimJSON:(NSString*) claimJSON
                 revocRegSeqNo:(NSNumber*) seqNo       // TODO: check how to deal with option<>
                userRevocIndex:(NSNumber*) revocIndex  // TODO: check how to deal with option<>
                    completion:(void (^)(NSError* error, NSString* revocRegUpdateJSON, NSString* claimJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_issuer_create_claim(handle,
                                     walletHandle,
                                     [reqJSON UTF8String],
                                     [claimJSON UTF8String],
                                     seqNo ? [seqNo intValue] : -1,
                                     revocIndex ? [revocIndex intValue] : -1,
                                     SovrinWrapperCommon4PCallback
                                    );

    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) issuerRevokeClaim:(SovrinHandle) walletHandle
                 claimDefSeqNo:(NSNumber*) claimSeqNo
                 revocRegSeqNo:(NSNumber*) revocSeqNo
                userRevocIndex:(NSNumber*) revocIndex
                    completion:(void (^)(NSError* error, NSString* revocRegUpdateJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_issuer_revoke_claim(handle,
                                     walletHandle,
                                     [claimSeqNo intValue],
                                     [revocSeqNo intValue],
                                     [revocIndex intValue],
                                     SovrinWrapperCommon3PSCallback
                                    );
    
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) proverStoreClaimOffer:(SovrinHandle) walletHandle
                    claimOfferJSON:(NSString*) json
                        completion:(void (^)(NSError* error)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_prover_store_claim_offer(handle,
                                          walletHandle,
                                          [json UTF8String],
                                          SovrinWrapperCommon2PCallback
                                         );
    
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) proverGetClaimOffers:(SovrinHandle) walletHandle
                       filterJSON:(NSString*) json
                       completion:(void (^)(NSError* error, NSString* claimOffersJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_prover_get_claim_offers(handle,
                                         walletHandle,
                                         [json UTF8String],
                                         SovrinWrapperCommon3PSCallback
                                        );
    
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) proverCreateMasterSecret:(SovrinHandle) walletHandle
                     masterSecretName:(NSString*) name
                           completion:(void (^)(NSError* error)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_prover_create_master_secret(handle,
                                             walletHandle,
                                             [name UTF8String],
                                             SovrinWrapperCommon2PCallback
                                            );

    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) proverCreateAndStoreClaimReq:(SovrinHandle) walletHandle
                                proverDid:(NSString*) prover
                           claimOfferJSON:(NSString*) offerJson
                         masterSecretName:(NSString*) name
                             claimDefJSON:(NSString*) claimJson
                               completion:(void (^)(NSError* error, NSString* claimReqJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];

    ret = sovrin_prover_create_and_store_claim_req(handle,
                                                   walletHandle,
                                                   [prover UTF8String],
                                                   [offerJson UTF8String],
                                                   [claimJson UTF8String],
                                                   [name UTF8String],
                                                   SovrinWrapperCommon3PSCallback
                                                  );
    
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) proverStoreClaim:(SovrinHandle) walletHandle
                   claimsJSON:(NSString*) claimsJson
                   completion:(void (^)(NSError* error)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_prover_store_claim(handle,
                                    walletHandle,
                                    [claimsJson UTF8String],
                                    SovrinWrapperCommon2PCallback
                                   );
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) proverGetClaims:(SovrinHandle) walletHandle
                  filterJSON:(NSString*) json
                  completion:(void (^)(NSError* error, NSString* claimsJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_prover_get_claims(handle,
                                   walletHandle,
                                   [json UTF8String],
                                   SovrinWrapperCommon3PSCallback
                                  );
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) proverGetClaimsForProofReq:(SovrinHandle) walletHandle
                           proofReqJSON:(NSString*) json
                             completion:(void (^)(NSError* error, NSString* claimsJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_prover_get_claims_for_proof_req(handle,
                                                 walletHandle,
                                                 [json UTF8String],
                                                 SovrinWrapperCommon3PSCallback
                                                );
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) proverCreateProof:(SovrinHandle) walletHandle
                  proofReqJSON:(NSString*) reqJSON
           requestedClaimsJSON:(NSString*) claimsJSON
                   schemasJSON:(NSString*) schemasJSON
              masterSecretName:(NSString*) name
                  claimDefsJSON:(NSString*) claimDefsJSON
                 revocRegsJSON:(NSString*) revocJSON
                    completion:(void (^)(NSError* error, NSString* proofJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_prover_create_proof(handle,
                                     walletHandle,
                                     [reqJSON UTF8String],
                                     [claimsJSON UTF8String],
                                     [schemasJSON UTF8String],
                                     [name UTF8String],
                                     [claimDefsJSON UTF8String],
                                     [revocJSON UTF8String],
                                     SovrinWrapperCommon3PSCallback
                                    );
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) verifierVerifyProof:(SovrinHandle) walletHandle
                    proofReqJSON:(NSString*) reqJSON
                       proofJSON:(NSString*) proofJSON
                     schemasJSON:(NSString*) schemasJSON
                   claimDefsJSON:(NSString*) claimDefsJSON
                   revocRegsJSON:(NSString*) revocJSON
                      completion:(void (^)(NSError* error, BOOL valid)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];

    ret = sovrin_verifier_verify_proof(handle,
                                       walletHandle,
                                       [reqJSON UTF8String],
                                       [proofJSON UTF8String],
                                       [schemasJSON UTF8String],
                                       [claimDefsJSON UTF8String],
                                       [revocJSON UTF8String],
                                       SovrinWrapperCommon3PBCallback
                                      );
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];

}

@end
