//
//  IndyAnoncreds.m
//  libindy
//

#import "IndyAnoncreds.h"
#import "IndyCallbacks.h"
#import "indy_core.h"
#import "NSError+IndyError.h"
#import "IndyTypes.h"

@implementation IndyAnoncreds

+ (NSError *)issuerCreateAndStoreClaimDefWithWalletHandle:(IndyHandle)walletHandle
                                                issuerDid:(NSString *)issuerDid
                                               schemaJSON:(NSString *)schema
                                            signatureType:(NSString *)signatureType
                                           createNonRevoc:(BOOL)createNonRevoc
                                               completion:(void (^)(NSError *error, NSString *claimDefJSON)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_issuer_create_and_store_claim_def(handle,
                                                 walletHandle,
                                                 [issuerDid UTF8String],
                                                 [schema UTF8String],
                                                 [signatureType UTF8String],
                                                 (indy_bool_t) createNonRevoc,
                                                 IndyWrapperCommon3PSCallback);
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)issuerCreateAndStoreRevocRegWithWalletHandle:(IndyHandle)walletHandle
                                                issuerDid:(NSString *)issuerDid
                                              schemaSeqNo:(NSNumber *)schemaSeqNo
                                              maxClaimNum:(NSNumber *)maxClaimNum
                                               completion:(void (^)(NSError *error, NSString *revocRegJSON, NSString *revocRegUUID)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_issuer_create_and_store_revoc_reg(handle,
                                                 walletHandle,
                                                 [issuerDid UTF8String],
                                                 [schemaSeqNo intValue],
                                                 [maxClaimNum intValue],
                                                 IndyWrapperCommon4PCallback);
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)issuerCreateClaimWithWalletHandle:(IndyHandle)walletHandle
                                  claimReqJSON:(NSString *)claimReqJSON
                                     claimJSON:(NSString *)claimJSON
                                userRevocIndex:(NSNumber *)userRevocIndex
                                    completion:(void (^)(NSError *error, NSString *revocRegUpdateJSON, NSString *xclaimJSON)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];

    ret = indy_issuer_create_claim(handle,
                                   walletHandle,
                                   [claimReqJSON UTF8String],
                                   [claimJSON UTF8String],
                                   userRevocIndex ? [userRevocIndex intValue] : -1,
                                   IndyWrapperCommon4PCallback);

    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)issuerRevokeClaimWithWalletHandle:(IndyHandle)walletHandle
                                     issuerDid:(NSString *)issuerDid
                                   schemaSeqNo:(NSNumber *)schemaSeqNo
                                userRevocIndex:(NSNumber *)userRevocIndex
                                    completion:(void (^)(NSError *error, NSString *revocRegUpdateJSON)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_issuer_revoke_claim(handle,
                                   walletHandle,
                                   [issuerDid UTF8String],
                                   schemaSeqNo ? [schemaSeqNo intValue] : -1,
                                   userRevocIndex ? [userRevocIndex intValue] : -1,
                                   IndyWrapperCommon3PSCallback);
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)proverStoreClaimOfferWithWalletHandle:(IndyHandle)walletHandle
                                    claimOfferJSON:(NSString *)claimOfferJSON
                                        completion:(void (^)(NSError *error)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_prover_store_claim_offer(handle,
                                        walletHandle,
                                        [claimOfferJSON UTF8String],
                                        IndyWrapperCommon2PCallback
                                        );
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)proverGetClaimOffersWithWalletHandle:(IndyHandle)walletHandle
                                       filterJSON:(NSString *)filterJSON
                                       completion:(void (^)(NSError *error, NSString *claimOffersJSON)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_prover_get_claim_offers(handle,
                                       walletHandle,
                                       [filterJSON UTF8String],
                                       IndyWrapperCommon3PSCallback
                                       );
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)proverCreateMasterSecretWithWalletHandle:(IndyHandle)walletHandle
                                     masterSecretName:(NSString *)masterSecretName
                                           completion:(void (^)(NSError *error)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_prover_create_master_secret(handle,
                                           walletHandle,
                                           [masterSecretName UTF8String],
                                           IndyWrapperCommon2PCallback
                                           );

    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)proverCreateAndStoreClaimReqWithWalletHandle:(IndyHandle)walletHandle
                                                proverDid:(NSString *)proverDid
                                           claimOfferJSON:(NSString *)claimOfferJSON
                                             claimDefJSON:(NSString *)claimDefJSON
                                         masterSecretName:(NSString *)masterSecretName
                                               completion:(void (^)(NSError *error, NSString *claimReqJSON)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];

    ret = indy_prover_create_and_store_claim_req(handle,
                                                 walletHandle,
                                                 [proverDid UTF8String],
                                                 [claimOfferJSON UTF8String],
                                                 [claimDefJSON UTF8String],
                                                 [masterSecretName UTF8String],
                                                 IndyWrapperCommon3PSCallback
                                                 );
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)proverStoreClaimWithWalletHandle:(IndyHandle)walletHandle
                                   claimsJSON:(NSString *)claimsJson
                                   completion:(void (^)(NSError *error)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_prover_store_claim(handle,
                                  walletHandle,
                                  [claimsJson UTF8String],
                                  IndyWrapperCommon2PCallback
                                  );
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)proverGetClaimsWithWalletHandle:(IndyHandle) walletHandle
                                  filterJSON:(NSString *)filterJSON
                                  completion:(void (^)(NSError *error, NSString *claimsJSON)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_prover_get_claims(handle,
                                 walletHandle,
                                 [filterJSON UTF8String],
                                 IndyWrapperCommon3PSCallback
                                 );
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)proverGetClaimsForProofReqWithWalletHandle:(IndyHandle)walletHandle
                                           proofReqJSON:(NSString *)proofReqJSON
                                             completion:(void (^)(NSError *error, NSString *claimsJSON)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_prover_get_claims_for_proof_req(handle,
                                               walletHandle,
                                               [proofReqJSON UTF8String],
                                               IndyWrapperCommon3PSCallback
                                               );
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)proverCreateProofWithWalletHandle:(IndyHandle)walletHandle
                                  proofReqJSON:(NSString *)proofReqJSON
                           requestedClaimsJSON:(NSString *)requestedClaimsJSON
                                   schemasJSON:(NSString *)schemasJSON
                              masterSecretName:(NSString *)masterSecretName
                                 claimDefsJSON:(NSString *)claimDefsJSON
                                 revocRegsJSON:(NSString *)revocRegsJSON
                                    completion:(void (^)(NSError *error, NSString *proofJSON)) handler;
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_prover_create_proof(handle,
                                   walletHandle,
                                   [proofReqJSON UTF8String],
                                   [requestedClaimsJSON UTF8String],
                                   [schemasJSON UTF8String],
                                   [masterSecretName UTF8String],
                                   [claimDefsJSON UTF8String],
                                   [revocRegsJSON UTF8String],
                                   IndyWrapperCommon3PSCallback
                                   );
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)verifierVerifyProofWithWalletHandle:(NSString *)proofReqJSON
                                       proofJSON:(NSString *)proofJSON
                                     schemasJSON:(NSString *)schemasJSON
                                   claimDefsJSON:(NSString *)claimDefsJSON
                                   revocRegsJSON:(NSString *)revocRegsJSON
                                      completion:(void (^)(NSError *error, BOOL valid)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];

    ret = indy_verifier_verify_proof(handle,
                                     [proofReqJSON UTF8String],
                                     [proofJSON UTF8String],
                                     [schemasJSON UTF8String],
                                     [claimDefsJSON UTF8String],
                                     [revocRegsJSON UTF8String],
                                     IndyWrapperCommon3PBCallback
                                     );
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

@end
