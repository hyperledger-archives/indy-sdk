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

+ (void)issuerCreateAndStoreCredentialDefForIssuerDID:(NSString *)issuerDID
                                           schemaJSON:(NSString *)schemaJSON
                                                  tag:(NSString *)tag
                                                 type:(NSString *)type
                                           configJSON:(NSString *)configJSON
                                         walletHandle:(IndyHandle)walletHandle
                                           completion:(void (^)(NSError *error, NSString *credentialDefId, NSString *credentialDefJSON))completion; {
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

+ (void)issuerCreateCredentialOfferForProverDID:(NSString *)proverDID
                                      issuerDID:(NSString *)issuerDID
                                      credDefId:(NSString *)credDefId
                                   walletHandle:(IndyHandle)walletHandle
                                     completion:(void (^)(NSError *error, NSString *credentialOfferJSON))completion; {
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


+ (void)issuerCreateCredentialWithRequest:(NSString *)credentialRequestJSON
                     credentialValuesJSON:(NSString *)credentialValuesJSON
                                 revRegId:(NSString *)revRegId
                        tailsReaderHandle:(NSNumber *)tailsReaderHandle
                           userRevocIndex:(NSNumber *)userRevocIndex
                             walletHandle:(IndyHandle)walletHandle
                               completion:(void (^)(NSError *error, NSString *revocRegDeltaJSON, NSString *xcredentialJSON))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_issuer_create_credential(handle,
            walletHandle,
            [credentialRequestJSON UTF8String],
            [credentialValuesJSON UTF8String],
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

+ (void)issuerRevokeCredentialForRevRegId:(NSString *)revRegId
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

+ (void)issuerRecoverCredentialForRevRegId:(NSString *)revRegId
                         tailsReaderHandle:(NSNumber *)tailsReaderHandle
                            userRevocIndex:(NSNumber *)userRevocIndex
                              walletHandle:(IndyHandle)walletHandle
                                completion:(void (^)(NSError *error, NSString *revocRegDeltaJSON))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_issuer_recover_credential(handle,
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

+ (void)proverStoreCredentialOffer:(NSString *)credentialOfferJSON
                  WithWalletHandle:(IndyHandle)walletHandle
                        completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_store_credential_offer(handle,
            walletHandle,
            [credentialOfferJSON UTF8String],
            IndyWrapperCommon2PCallback
    );

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret]);
        });
    }
}

+ (void)proverGetCredentialOffersWithFilter:(NSString *)filterJSON
                               walletHandle:(IndyHandle)walletHandle
                                 completion:(void (^)(NSError *error, NSString *credentialOffersJSON))completion {
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

+ (void)proverCreateAndStoreCredentialReqWithCredentialDef:(NSString *)credentialDefJSON
                                                 proverDID:(NSString *)proverDID
                                       credentialOfferJSON:(NSString *)credentialOfferJSON
                                          masterSecretName:(NSString *)masterSecretName
                                              walletHandle:(IndyHandle)walletHandle
                                                completion:(void (^)(NSError *error, NSString *credentialReqJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_create_and_store_credential_req(handle,
            walletHandle,
            [proverDID UTF8String],
            [credentialOfferJSON UTF8String],
            [credentialDefJSON UTF8String],
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

+ (void)proverStoreCredential:(NSString *)credentialsJson
                 credentialId:(NSString *)credentialId
                revRegDefJSON:(NSString *)revRegDefJSON
                 walletHandle:(IndyHandle)walletHandle
                   completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_store_credential(handle,
            walletHandle,
            [credentialId UTF8String],
            [credentialsJson UTF8String],
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

+ (void)proverGetCredentialsWithFilter:(NSString *)filterJSON
                          walletHandle:(IndyHandle)walletHandle
                            completion:(void (^)(NSError *error, NSString *credentialsJSON))completion {
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

+ (void)proverGetCredentialsForProofReq:(NSString *)proofReqJSON
                           walletHandle:(IndyHandle)walletHandle
                             completion:(void (^)(NSError *error, NSString *credentialsJSON))completion {
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
           requestedCredentialsJSON:(NSString *)requestedCredentialsJSON
                        schemasJSON:(NSString *)schemasJSON
                   masterSecretName:(NSString *)masterSecretName
                 credentialDefsJSON:(NSString *)credentialDefsJSON
                     revocInfosJSON:(NSString *)revocInfosJSON
                       walletHandle:(IndyHandle)walletHandle
                         completion:(void (^)(NSError *error, NSString *proofJSON))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_create_proof(handle,
            walletHandle,
            [proofRequestJSON UTF8String],
            [requestedCredentialsJSON UTF8String],
            [schemasJSON UTF8String],
            [masterSecretName UTF8String],
            [credentialDefsJSON UTF8String],
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
                credentialDefsJSON:(NSString *)credentialDefsJSON
                  revocRegDefsJSON:(NSString *)revocRegDefsJSON
                     revocRegsJSON:(NSString *)revocRegsJSON
                        completion:(void (^)(NSError *error, BOOL valid))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_verifier_verify_proof(handle,
            [proofRequestJson UTF8String],
            [proofJSON UTF8String],
            [schemasJSON UTF8String],
            [credentialDefsJSON UTF8String],
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

+ (void)createRevocationInfoForTimestamp:(NSNumber *)timestamp
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

+ (void)updateRevocationInfoForTimestamp:(NSNumber *)timestamp
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

+ (void)storeRevocationInfoForId:(NSString *)id
                           revInfoJSON:(NSString *)revInfoJSON
                          walletHandle:(IndyHandle)walletHandle
                            completion:(void (^)(NSError *error))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_store_revocation_info(handle,
            walletHandle,
            [id UTF8String],
            [revInfoJSON UTF8String],
            IndyWrapperCommon2PCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret]);
        });
    }
}

+ (void)getRevocationInfoForId:(NSString *)id
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
