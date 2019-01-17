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

+ (void)issuerCreateSchemaWithName:(NSString *)name
                           version:(NSString *)version
                             attrs:(NSString *)attrs
                         issuerDID:(NSString *)issuerDID
                        completion:(void (^)(NSError *error, NSString *schemaId, NSString *schemaJSON))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_issuer_create_schema(handle,
            [issuerDID UTF8String],
            [name UTF8String],
            [version UTF8String],
            [attrs UTF8String],
            IndyWrapperCommonStringStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil);
        });
    }
}

+ (void)issuerCreateAndStoreCredentialDefForSchema:(NSString *)schemaJSON
                                         issuerDID:(NSString *)issuerDID
                                               tag:(NSString *)tag
                                              type:(NSString *)type
                                        configJSON:(NSString *)configJSON
                                      walletHandle:(IndyHandle)walletHandle
                                        completion:(void (^)(NSError *error, NSString *credDefId, NSString *credDefJSON))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_issuer_create_and_store_credential_def(handle,
            walletHandle,
            [issuerDID UTF8String],
            [schemaJSON UTF8String],
            [tag UTF8String],
            [type UTF8String],
            [configJSON UTF8String],
            IndyWrapperCommonStringStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil);
        });
    }
}

+ (void)issuerCreateAndStoreRevocRegForCredentialDefId:(NSString *)credDefId
                                             issuerDID:(NSString *)issuerDID
                                                  type:(NSString *)type
                                                   tag:(NSString *)tag
                                            configJSON:(NSString *)configJSON
                                     tailsWriterHandle:(IndyHandle)tailsWriterHandle
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
            tailsWriterHandle,
            IndyWrapperCommonStringStringStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil, nil);
        });
    }
}

+ (void)issuerCreateCredentialOfferForCredDefId:(NSString *)credDefId
                                   walletHandle:(IndyHandle)walletHandle
                                     completion:(void (^)(NSError *error, NSString *credOfferJSON))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_issuer_create_credential_offer(handle,
            walletHandle,
            [credDefId UTF8String],
            IndyWrapperCommonStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}


+ (void)issuerCreateCredentialForCredentialRequest:(NSString *)credReqJSON
                                     credOfferJSON:(NSString *)credOfferJSON
                                    credValuesJSON:(NSString *)credValuesJSON
                                          revRegId:(NSString *)revRegId
                           blobStorageReaderHandle:(NSNumber *)blobStorageReaderHandle
                                      walletHandle:(IndyHandle)walletHandle
                                        completion:(void (^)(NSError *error, NSString *credJSON, NSString *credRevocID, NSString *revocRegDeltaJSON))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_issuer_create_credential(handle,
            walletHandle,
            [credOfferJSON UTF8String],
            [credReqJSON UTF8String],
            [credValuesJSON UTF8String],
            [revRegId UTF8String],
            blobStorageReaderHandle ? [blobStorageReaderHandle intValue] : -1,
            IndyWrapperCommonStringOptStringOptStringCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil, nil);
        });
    }
}

+ (void)issuerRevokeCredentialByCredRevocId:(NSString *)credRevocId
                                   revRegId:(NSString *)revRegId
                    blobStorageReaderHandle:(NSNumber *)blobStorageReaderHandle
                               walletHandle:(IndyHandle)walletHandle
                                 completion:(void (^)(NSError *error, NSString *revocRegDeltaJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_issuer_revoke_credential(handle,
            walletHandle,
            [blobStorageReaderHandle intValue],
            [revRegId UTF8String],
            [credRevocId UTF8String],
            IndyWrapperCommonStringCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

/*+ (void)issuerRecoverCredentialByCredRevocId:(NSString *)credRevocId
                                    revRegId:(NSString *)revRegId
                     blobStorageReaderHandle:(NSNumber *)blobStorageReaderHandle
                                walletHandle:(IndyHandle)walletHandle
                                  completion:(void (^)(NSError *error, NSString *revocRegDeltaJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_issuer_recover_credential(handle,
            walletHandle,
            [blobStorageReaderHandle intValue],
            [revRegId UTF8String],
            [credRevocId UTF8String],
            IndyWrapperCommonStringCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}*/

+ (void)issuerMergerRevocationRegistryDelta:(NSString *)revRegDelta
                                  withDelta:(NSString *)otherRevRegDelta
                                 completion:(void (^)(NSError *error, NSString *credOfferJSON))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_issuer_merge_revocation_registry_deltas(handle,
            [revRegDelta UTF8String],
            [otherRevRegDelta UTF8String],
            IndyWrapperCommonStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)proverCreateMasterSecret:(NSString *)masterSecretID
                    walletHandle:(IndyHandle)walletHandle
                      completion:(void (^)(NSError *error, NSString *outMasterSecretId))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_create_master_secret(handle,
            walletHandle,
            [masterSecretID UTF8String],
            IndyWrapperCommonStringCallback
    );

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)proverCreateCredentialReqForCredentialOffer:(NSString *)credOfferJSON
                                  credentialDefJSON:(NSString *)credentialDefJSON
                                          proverDID:(NSString *)proverDID
                                     masterSecretID:(NSString *)masterSecretID
                                       walletHandle:(IndyHandle)walletHandle
                                         completion:(void (^)(NSError *error, NSString *credReqJSON, NSString *credReqMetadataJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_create_credential_req(handle,
            walletHandle,
            [proverDID UTF8String],
            [credOfferJSON UTF8String],
            [credentialDefJSON UTF8String],
            [masterSecretID UTF8String],
            IndyWrapperCommonStringStringCallback
    );

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil);
        });
    }
}

+ (void)proverStoreCredential:(NSString *)credJson
                       credID:(NSString *)credID
          credReqMetadataJSON:(NSString *)credReqMetadataJSON
                  credDefJSON:(NSString *)credDefJSON
                revRegDefJSON:(NSString *)revRegDefJSON
                 walletHandle:(IndyHandle)walletHandle
                   completion:(void (^)(NSError *error, NSString *outCredID))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_store_credential(handle,
            walletHandle,
            [credID UTF8String],
            [credReqMetadataJSON UTF8String],
            [credJson UTF8String],
            [credDefJSON UTF8String],
            [revRegDefJSON UTF8String],
            IndyWrapperCommonStringCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)proverGetCredentialWithId:(NSString *)credId
                     walletHandle:(IndyHandle)walletHandle
                       completion:(void (^)(NSError *error, NSString *credentialJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_get_credential(handle,
            walletHandle,
            [credId UTF8String],
            IndyWrapperCommonStringCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)proverGetCredentialsForFilter:(NSString *)filterJSON
                         walletHandle:(IndyHandle)walletHandle
                           completion:(void (^)(NSError *error, NSString *credentialsJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_get_credentials(handle,
            walletHandle,
            [filterJSON UTF8String],
            IndyWrapperCommonStringCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)proverSearchCredentialsForQuery:(NSString *)queryJSON
                            walletHandle:(IndyHandle)walletHandle
                              completion:(void (^)(NSError *error, IndyHandle searchHandle, NSNumber *totalCount))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_search_credentials(handle,
            walletHandle,
            [queryJSON UTF8String],
            IndyWrapperCommonHandleNumberCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], 0, nil);
        });
    }
}

+ (void)proverFetchCredentialsWithSearchHandle:(IndyHandle)searchHandle
                                         count:(NSNumber *)count
                                    completion:(void (^)(NSError *error, NSString *credentialsJson))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_fetch_credentials(handle,
            searchHandle,
            [count unsignedIntValue],
            IndyWrapperCommonStringCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)proverCloseCredentialsSearchWithHandle:(IndyHandle)searchHandle
                                    completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_close_credentials_search(handle,
            searchHandle,
            IndyWrapperCommonCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret]);
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
            IndyWrapperCommonStringCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)proverSearchCredentialsForProofRequest:(NSString *)proofRequest
                                extraQueryJSON:(NSString *)extraQueryJSON
                                  walletHandle:(IndyHandle)walletHandle
                                    completion:(void (^)(NSError *error, IndyHandle searchHandle))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_search_credentials_for_proof_req(handle,
            walletHandle,
            [proofRequest UTF8String],
            [extraQueryJSON UTF8String],
            IndyWrapperCommonHandleCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], 0);
        });
    }
}

+ (void)proverFetchCredentialsForProofReqItemReferent:(NSString *)itemReferent
                                         searchHandle:(IndyHandle)searchHandle
                                                count:(NSNumber *)count
                                           completion:(void (^)(NSError *error, NSString *credentialsJson))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_fetch_credentials_for_proof_req(handle,
            searchHandle,
            [itemReferent UTF8String],
            [count unsignedIntValue],
            IndyWrapperCommonStringCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)proverCloseCredentialsSearchForProofReqWithHandle:(IndyHandle)searchHandle
                                               completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_close_credentials_search_for_proof_req(handle,
            searchHandle,
            IndyWrapperCommonCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret]);
        });
    }
}

+ (void)proverCreateProofForRequest:(NSString *)proofRequestJSON
           requestedCredentialsJSON:(NSString *)requestedCredentialsJSON
                     masterSecretID:(NSString *)masterSecretID
                        schemasJSON:(NSString *)schemasJSON
                 credentialDefsJSON:(NSString *)credentialDefsJSON
                    revocStatesJSON:(NSString *)revocStatesJSON
                       walletHandle:(IndyHandle)walletHandle
                         completion:(void (^)(NSError *error, NSString *proofJSON))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prover_create_proof(handle,
            walletHandle,
            [proofRequestJSON UTF8String],
            [requestedCredentialsJSON UTF8String],
            [masterSecretID UTF8String],
            [schemasJSON UTF8String],
            [credentialDefsJSON UTF8String],
            [revocStatesJSON UTF8String],
            IndyWrapperCommonStringCallback
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
            IndyWrapperCommonBoolCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], false);
        });
    }
}

+ (void)createRevocationStateForCredRevID:(NSString *)credRevID
                                timestamp:(NSNumber *)timestamp
                            revRegDefJSON:(NSString *)revRegDefJSON
                          revRegDeltaJSON:(NSString *)revRegDeltaJSON
                  blobStorageReaderHandle:(NSNumber *)blobStorageReaderHandle
                               completion:(void (^)(NSError *error, NSString *revStateJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_create_revocation_state(handle,
            [blobStorageReaderHandle intValue],
            [revRegDefJSON UTF8String],
            [revRegDeltaJSON UTF8String],
            [timestamp unsignedIntValue],
            [credRevID UTF8String],
            IndyWrapperCommonStringCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)updateRevocationState:(NSString *)revStateJSON
                    credRevID:(NSString *)credRevID
                    timestamp:(NSNumber *)timestamp
                revRegDefJSON:(NSString *)revRegDefJSON
              revRegDeltaJSON:(NSString *)revRegDeltaJSON
      blobStorageReaderHandle:(NSNumber *)blobStorageReaderHandle
                   completion:(void (^)(NSError *error, NSString *updatedRevStateJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_update_revocation_state(handle,
            [blobStorageReaderHandle intValue],
            [revStateJSON UTF8String],
            [revRegDefJSON UTF8String],
            [revRegDeltaJSON UTF8String],
            [timestamp unsignedIntValue],
            [credRevID UTF8String],
            IndyWrapperCommonStringCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

@end
