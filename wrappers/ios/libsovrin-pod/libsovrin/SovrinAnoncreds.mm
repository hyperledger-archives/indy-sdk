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

+ (NSError *)issuerCreateAndStoreClaimDefWithWalletHandle:(SovrinHandle)walletHandle
                                                issuerDid:(NSString *)issuerDid
                                               schemaJSON:(NSString *)schema
                                            signatureType:(NSString *)signatureType
                                           createNonRevoc:(BOOL)createNonRevoc
                                               completion:(void (^)(NSError *error, NSString *claimDefJSON, NSString *claimDefUUID)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_issuer_create_and_store_claim_def(handle,
                                                   walletHandle,
                                                   [issuerDid UTF8String],
                                                   [schema UTF8String],
                                                   [signatureType UTF8String],
                                                   (sovrin_bool_t) createNonRevoc,
                                                   SovrinWrapperCommon3PSCallback);
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError *)issuerCreateAndStoreRevocRegWithWalletHandle:(SovrinHandle)walletHandle
                                                issuerDid:(NSString *)issuerDid
                                            claimDefSeqNo:(NSNumber *)seqNo
                                              maxClaimNum:(NSNumber *)maxClaimNum
                                               completion:(void (^)(NSError *error, NSString *revocRegJSON, NSString *revocRegUUID)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_issuer_create_and_store_revoc_reg(handle,
                                                   walletHandle,
                                                   [issuerDid UTF8String],
                                                   [seqNo intValue],
                                                   [maxClaimNum intValue],
                                                   SovrinWrapperCommon4PCallback);
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError *)issuerCreateClaimWithWalletHandle:(SovrinHandle)walletHandle
                                  claimReqJSON:(NSString *)reqJSON
                                     claimJSON:(NSString *)claimJSON
                                 revocRegSeqNo:(NSNumber *)seqNo       // TODO: check how to deal with option<>
                                userRevocIndex:(NSNumber *)revocIndex  // TODO: check how to deal with option<>
                                    completion:(void (^)(NSError *error, NSString *revocRegUpdateJSON, NSString *claimJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
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
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError *)issuerRevokeClaimWithWalletHandle:(SovrinHandle)walletHandle
                                 revocRegSeqNo:(NSNumber *)revocSeqNo
                                userRevocIndex:(NSNumber *)revocIndex
                                    completion:(void (^)(NSError *error, NSString *revocRegUpdateJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_issuer_revoke_claim(handle,
                                     walletHandle,
                                     [revocSeqNo intValue],
                                     [revocIndex intValue],
                                     SovrinWrapperCommon3PSCallback
                                    );
    
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError *)proverStoreClaimOfferWithWalletHandle:(SovrinHandle)walletHandle
                                    claimOfferJSON:(NSString *)json
                                        completion:(void (^)(NSError *error)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_prover_store_claim_offer(handle,
                                          walletHandle,
                                          [json UTF8String],
                                          SovrinWrapperCommon2PCallback
                                         );
    
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError *)proverGetClaimOffersWithWalletHandle:(SovrinHandle)walletHandle
                                       filterJSON:(NSString *)json
                                       completion:(void (^)(NSError *error, NSString *claimOffersJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_prover_get_claim_offers(handle,
                                         walletHandle,
                                         [json UTF8String],
                                         SovrinWrapperCommon3PSCallback
                                        );
    
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError *)proverCreateMasterSecretWithWalletHandle:(SovrinHandle)walletHandle
                                     masterSecretName:(NSString *)name
                                           completion:(void (^)(NSError *error)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_prover_create_master_secret(handle,
                                             walletHandle,
                                             [name UTF8String],
                                             SovrinWrapperCommon2PCallback
                                            );

    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError *)proverCreateAndStoreClaimReqWithWalletHandle:(SovrinHandle)walletHandle
                                                proverDid:(NSString *)prover
                                           claimOfferJSON:(NSString *)offerJson
                                             claimDefJSON:(NSString *)claimJson
                                         masterSecretName:(NSString *)name
                                               completion:(void (^)(NSError *error, NSString *claimReqJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];

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
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError *)proverStoreClaimWithWalletHandle:(SovrinHandle)walletHandle
                                   claimsJSON:(NSString *)claimsJson
                                   completion:(void (^)(NSError *error)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_prover_store_claim(handle,
                                    walletHandle,
                                    [claimsJson UTF8String],
                                    SovrinWrapperCommon2PCallback
                                   );
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError *)proverGetClaimsWithWalletHandle:(SovrinHandle) walletHandle
                                  filterJSON:(NSString *)json
                                  completion:(void (^)(NSError *error, NSString *claimsJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_prover_get_claims(handle,
                                   walletHandle,
                                   [json UTF8String],
                                   SovrinWrapperCommon3PSCallback
                                  );
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError *)proverGetClaimsForProofReqWithWalletHandle:(SovrinHandle)walletHandle
                                           proofReqJSON:(NSString *)json
                                             completion:(void (^)(NSError *error, NSString *claimsJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_prover_get_claims_for_proof_req(handle,
                                                 walletHandle,
                                                 [json UTF8String],
                                                 SovrinWrapperCommon3PSCallback
                                                );
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError *)proverCreateProofWithWalletHandle:(SovrinHandle)walletHandle
                                  proofReqJSON:(NSString *)reqJSON
                           requestedClaimsJSON:(NSString *)claimsJSON
                                   schemasJSON:(NSString *)schemasJSON
                              masterSecretName:(NSString *)name
                                 claimDefsJSON:(NSString *)claimDefsJSON
                                 revocRegsJSON:(NSString *)revocJSON
                                    completion:(void (^)(NSError *error, NSString *proofJSON)) handler;
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
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
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError *)verifierVerifyProofWithWalletHandle:(NSString *)proofReqJSON
                                       proofJSON:(NSString *)proofJSON
                                     schemasJSON:(NSString *)schemasJSON
                                   claimDefsJSON:(NSString *)claimDefsJSON
                                   revocRegsJSON:(NSString *)revocJSON
                                      completion:(void (^)(NSError *error, BOOL valid)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];

    ret = sovrin_verifier_verify_proof(handle,
                                       [proofReqJSON UTF8String],
                                       [proofJSON UTF8String],
                                       [schemasJSON UTF8String],
                                       [claimDefsJSON UTF8String],
                                       [revocJSON UTF8String],
                                       SovrinWrapperCommon3PBCallback
                                      );
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];

}

@end
