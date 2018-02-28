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

+ (void)issuerCreateAndStoreClaimDefForIssuerDID:(NSString *)issuerDID
                                      schemaJSON:(NSString *)schemaJSON
                                   signatureType:(NSString *)signatureType
                                  createNonRevoc:(BOOL)createNonRevoc
                                    walletHandle:(IndyHandle)walletHandle
                                      completion:(void (^)(NSError *error, NSString *claimDefJSON))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_issuer_create_and_store_claim_def(handle,
            walletHandle,
            [issuerDID UTF8String],
            [schemaJSON UTF8String],
            [signatureType UTF8String],
            (indy_bool_t) createNonRevoc,
            IndyWrapperCommon3PSCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)issuerCreateAndStoreRevocRegForIssuerDid:(NSString *)issuerDID
                                      schemaJSON:(NSString *)schemaJSON
                                     maxClaimNum:(NSNumber *)maxClaimNum
                                    walletHandle:(IndyHandle)walletHandle
                                      completion:(void (^)(NSError *error, NSString *revocRegJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_issuer_create_and_store_revoc_reg(handle,
            walletHandle,
            [issuerDID UTF8String],
            [schemaJSON UTF8String],
            [maxClaimNum intValue],
            IndyWrapperCommon3PSCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)issuerCreateClaimOfferForProverDID:(NSString *)proverDID
                                 issuerDID:(NSString *)issuerDID
                                schemaJSON:(NSString *)schemaJSON
                              walletHandle:(IndyHandle)walletHandle
                                completion:(void (^)(NSError *error, NSString *claimOfferJSON))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_issuer_create_claim_offer(handle,
            walletHandle,
            [schemaJSON UTF8String],
            [issuerDID UTF8String],
            [proverDID UTF8String],
            IndyWrapperCommon3PSCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}


+ (void)issuerCreateClaimWithRequest:(NSString *)claimRequestJSON
                           claimJSON:(NSString *)claimJSON
                      userRevocIndex:(NSNumber *)userRevocIndex
                        walletHandle:(IndyHandle)walletHandle
                          completion:(void (^)(NSError *error, NSString *revocRegUpdateJSON, NSString *xclaimJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_issuer_create_claim(handle,
            walletHandle,
            [claimRequestJSON UTF8String],
            [claimJSON UTF8String],
            userRevocIndex ? [userRevocIndex intValue] : -1,
            IndyWrapperCommon4PCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil);
        });
    }
}

+ (void)issuerRevokeClaimForIssuerDID:(NSString *)issuerDID
                           schemaJSON:(NSString *)schemaJSON
                       userRevocIndex:(NSNumber *)userRevocIndex
                         walletHandle:(IndyHandle)walletHandle
                           completion:(void (^)(NSError *error, NSString *revocRegUpdateJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_issuer_revoke_claim(handle,
            walletHandle,
            [issuerDID UTF8String],
            [schemaJSON UTF8String],
            userRevocIndex ? [userRevocIndex intValue] : -1,
            IndyWrapperCommon3PSCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)proverStoreClaimOffer:(NSString *)claimOfferJSON
             WithWalletHandle:(IndyHandle)walletHandle
                   completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_store_claim_offer(handle,
            walletHandle,
            [claimOfferJSON UTF8String],
            IndyWrapperCommon2PCallback
    );

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret]);
        });
    }
}

+ (void)proverGetClaimOffersWithFilter:(NSString *)filterJSON
                          walletHandle:(IndyHandle)walletHandle
                            completion:(void (^)(NSError *error, NSString *claimOffersJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_get_claim_offers(handle,
            walletHandle,
            [filterJSON UTF8String],
            IndyWrapperCommon3PSCallback
    );

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)proverCreateMasterSecretNamed:(NSString *)masterSecretName
                         walletHandle:(IndyHandle)walletHandle
                           completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_create_master_secret(handle,
            walletHandle,
            [masterSecretName UTF8String],
            IndyWrapperCommon2PCallback
    );

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret]);
        });
    }
}

+ (void)proverCreateAndStoreClaimReqWithClaimDef:(NSString *)claimDefJSON
                                       proverDID:(NSString *)proverDID
                                  claimOfferJSON:(NSString *)claimOfferJSON
                                masterSecretName:(NSString *)masterSecretName
                                    walletHandle:(IndyHandle)walletHandle
                                      completion:(void (^)(NSError *error, NSString *claimReqJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_create_and_store_claim_req(handle,
            walletHandle,
            [proverDID UTF8String],
            [claimOfferJSON UTF8String],
            [claimDefJSON UTF8String],
            [masterSecretName UTF8String],
            IndyWrapperCommon3PSCallback
    );

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)proverStoreClaim:(NSString *)claimsJson
              revRegJSON:(NSString *)revRegJSON
            walletHandle:(IndyHandle)walletHandle
              completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_store_claim(handle,
            walletHandle,
            [claimsJson UTF8String],
            [revRegJSON UTF8String],
            IndyWrapperCommon2PCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret]);
        });
    }
}

+ (void)proverGetClaimsWithFilter:(NSString *)filterJSON
                     walletHandle:(IndyHandle)walletHandle
                       completion:(void (^)(NSError *error, NSString *claimsJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_get_claims(handle,
            walletHandle,
            [filterJSON UTF8String],
            IndyWrapperCommon3PSCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)proverGetClaimsForProofReq:(NSString *)proofReqJSON
                      walletHandle:(IndyHandle)walletHandle
                        completion:(void (^)(NSError *error, NSString *claimsJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_get_claims_for_proof_req(handle,
            walletHandle,
            [proofReqJSON UTF8String],
            IndyWrapperCommon3PSCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)proverCreateProofForRequest:(NSString *)proofRequestJSON
                requestedClaimsJSON:(NSString *)requestedClaimsJSON
                        schemasJSON:(NSString *)schemasJSON
                   masterSecretName:(NSString *)masterSecretName
                      claimDefsJSON:(NSString *)claimDefsJSON
                      revocRegsJSON:(NSString *)revocRegsJSON
                       walletHandle:(IndyHandle)walletHandle
                         completion:(void (^)(NSError *error, NSString *proofJSON))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_create_proof(handle,
            walletHandle,
            [proofRequestJSON UTF8String],
            [requestedClaimsJSON UTF8String],
            [schemasJSON UTF8String],
            [masterSecretName UTF8String],
            [claimDefsJSON UTF8String],
            [revocRegsJSON UTF8String],
            IndyWrapperCommon3PSCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)verifierVerifyProofRequest:(NSString *)proofRequestJson
                         proofJSON:(NSString *)proofJSON
                       schemasJSON:(NSString *)schemasJSON
                     claimDefsJSON:(NSString *)claimDefsJSON
                     revocRegsJSON:(NSString *)revocRegsJSON
                        completion:(void (^)(NSError *error, BOOL valid))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_verifier_verify_proof(handle,
            [proofRequestJson UTF8String],
            [proofJSON UTF8String],
            [schemasJSON UTF8String],
            [claimDefsJSON UTF8String],
            [revocRegsJSON UTF8String],
            IndyWrapperCommon3PBCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], false);
        });
    }
}

@end
