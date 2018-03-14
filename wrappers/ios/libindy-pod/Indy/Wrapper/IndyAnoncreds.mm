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

+ (void)issuerCreateSchemaForIssuerDID:(NSString *)issuerDID
                                  name:(NSString *)name
                               version:(NSString *)version
                                 attrs:(NSString *)attrs
                            completion:(void (^)(NSError *error, NSString *schemaId, NSString *schemaJSON))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_issuer_create_schema(handle,
            [issuerDID UTF8String],
            [name UTF8String],
            [version UTF8String],
            [attrs UTF8String],
            IndyWrapperCommon4PCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil);
        });
    }
}

+ (void)issuerCreateAndStoreClaimDefForIssuerDID:(NSString *)issuerDID
                                      schemaJSON:(NSString *)schemaJSON
                                             tag:(NSString *)tag
                                            type:(NSString *)type
                                      configJSON:(NSString *)configJSON
                                    walletHandle:(IndyHandle)walletHandle
                                      completion:(void (^)(NSError *error, NSString *claimDefId, NSString *claimDefJSON))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_issuer_create_and_store_credential_def(handle,
            walletHandle,
            [issuerDID UTF8String],
            [schemaJSON UTF8String],
            [tag UTF8String],
            [type UTF8String],
            [configJSON UTF8String],
            IndyWrapperCommon4PCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil);
        });
    }
}

+ (void)issuerCreateAndStoreRevocRegForIssuerDid:(NSString *)issuerDID
                                            type:(NSString *)type
                                             tag:(NSString *)tag
                                       credDefId:(NSString *)credDefId
                                      configJSON:(NSString *)configJSON
                                 tailsWriterType:(NSString *)tailsWriterType
                               tailsWriterConfig:(NSString *)tailsWriterConfig
                                    walletHandle:(IndyHandle)walletHandle
                                      completion:(void (^)(NSError *error, NSString *revocRegID, NSString *revocRegDefJSON, NSString *revocRegEntryJSON))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_issuer_create_and_store_revoc_reg(handle,
            walletHandle,
            [issuerDID UTF8String],
            [type UTF8String],
            [tag UTF8String],
            [credDefId UTF8String],
            [configJSON UTF8String],
            [tailsWriterType UTF8String],
            [tailsWriterConfig UTF8String],
            IndyWrapperCommon5PCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil, nil);
        });
    }
}

+ (void)issuerCreateClaimOfferForProverDID:(NSString *)proverDID
                                 issuerDID:(NSString *)issuerDID
                                 credDefId:(NSString *)credDefId
                              walletHandle:(IndyHandle)walletHandle
                                completion:(void (^)(NSError *error, NSString *claimOfferJSON))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_issuer_create_credential_offer(handle,
            walletHandle,
            [credDefId UTF8String],
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
                     claimValuesJSON:(NSString *)claimValuesJSON
                            revRegId:(NSString *)revRegId
                   tailsReaderHandle:(NSNumber *)tailsReaderHandle
                      userRevocIndex:(NSNumber *)userRevocIndex
                        walletHandle:(IndyHandle)walletHandle
                          completion:(void (^)(NSError *error, NSString *revocRegDeltaJSON, NSString *xclaimJSON))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_issuer_create_credential(handle,
            walletHandle,
            [claimRequestJSON UTF8String],
            [claimValuesJSON UTF8String],
            [revRegId UTF8String],
            tailsReaderHandle ? [tailsReaderHandle intValue] : -1,
            userRevocIndex ? [userRevocIndex intValue] : -1,
            IndyWrapperCommon4PSCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil);
        });
    }
}

+ (void)issuerRevokeClaimForRevRegId:(NSString *)revRegId
                   tailsReaderHandle:(NSNumber *)tailsReaderHandle
                      userRevocIndex:(NSNumber *)userRevocIndex
                        walletHandle:(IndyHandle)walletHandle
                          completion:(void (^)(NSError *error, NSString *revocRegDeltaJSON))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_issuer_revoke_credential(handle,
            walletHandle,
            [tailsReaderHandle intValue],
            [revRegId UTF8String],
            [userRevocIndex intValue],
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

    ret = indy_prover_store_credential_offer(handle,
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

    ret = indy_prover_get_credential_offers(handle,
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

    ret = indy_prover_create_and_store_credential_req(handle,
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
                 claimId:(NSString *)claimId
           revRegDefJSON:(NSString *)revRegDefJSON
            walletHandle:(IndyHandle)walletHandle
              completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_store_credential(handle,
            walletHandle,
            [claimId UTF8String],
            [claimsJson UTF8String],
            [revRegDefJSON UTF8String],
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

    ret = indy_prover_get_credentials(handle,
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

    ret = indy_prover_get_credentials_for_proof_req(handle,
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
                     revocInfosJSON:(NSString *)revocInfosJSON
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
            [revocInfosJSON UTF8String],
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
                  revocRegDefsJSON:(NSString *)revocRegDefsJSON
                     revocRegsJSON:(NSString *)revocRegsJSON
                        completion:(void (^)(NSError *error, BOOL valid))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_verifier_verify_proof(handle,
            [proofRequestJson UTF8String],
            [proofJSON UTF8String],
            [schemasJSON UTF8String],
            [claimDefsJSON UTF8String],
            [revocRegDefsJSON UTF8String],
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

+ (void)issuerCreateRevocationInfoForTimestamp:(NSNumber *)timestamp
                                 revRegDefJSON:(NSString *)revRegDefJSON
                               revRegDeltaJSON:(NSString *)revRegDeltaJSON
                             tailsReaderHandle:(NSNumber *)tailsReaderHandle
                                        revIdx:(NSNumber *)revIdx
                                    completion:(void (^)(NSError *error, NSString *revInfo))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_create_revocation_info(handle,
            [tailsReaderHandle intValue],
            [revRegDefJSON UTF8String],
            [revRegDeltaJSON UTF8String],
            [timestamp intValue],
            [revIdx intValue],
            IndyWrapperCommon3PSCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)issuerUpdateRevocationInfoForTimestamp:(NSNumber *)timestamp
                                   revInfoJSON:(NSString *)revInfoJSON
                                 revRegDefJSON:(NSString *)revRegDefJSON
                               revRegDeltaJSON:(NSString *)revRegDeltaJSON
                             tailsReaderHandle:(NSNumber *)tailsReaderHandle
                                        revIdx:(NSNumber *)revIdx
                                    completion:(void (^)(NSError *error, NSString *updatedRevInfo))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_update_revocation_info(handle,
            [tailsReaderHandle intValue],
            [revInfoJSON UTF8String],
            [revRegDefJSON UTF8String],
            [revRegDeltaJSON UTF8String],
            [timestamp intValue],
            [revIdx intValue],
            IndyWrapperCommon3PSCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)issuerStoreRevocationInfoForId:(NSString *)id
                           revInfoJSON:(NSString *)revInfoJSON
                          walletHandle:(IndyHandle)walletHandle
                            completion:(void (^)(NSError *error))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_store_revocation_info(handle,
            walletHandle,
            [id UTF8String],
            [revInfoJSON UTF8String],
            IndyWrapperCommon3PSCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret]);
        });
    }
}

+ (void)issuerGetRevocationInfoForId:(NSString *)id
                           timestamp:(NSNumber *)timestamp
                        walletHandle:(IndyHandle)walletHandle
                          completion:(void (^)(NSError *error, NSString *revInfo))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_get_revocation_info(handle,
            walletHandle,
            [id UTF8String],
            timestamp ? [timestamp intValue] : -1,
            IndyWrapperCommon3PSCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

@end
