//
//  IndyLedger.m
//  libindy
//


#import "IndyLedger.h"
#import "IndyCallbacks.h"
#import "indy_core.h"
#import "NSError+IndyError.h"

@implementation IndyLedger


+ (void)signAndSubmitRequest:(NSString *)requestJSON
                submitterDID:(NSString *)submitterDid
                  poolHandle:(IndyHandle)poolHandle
                walletHandle:(IndyHandle)walletHandle
                  completion:(void (^)(NSError *error, NSString *requestResultJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_sign_and_submit_request(handle,
            poolHandle,
            walletHandle,
            [submitterDid UTF8String],
            [requestJSON UTF8String],
            IndyWrapperCommon3PSCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)signRequest:(NSString *)requestJson
       submitterDid:(NSString *)submitterDid
       walletHandle:(IndyHandle)walletHandle
         completion:(void (^)(NSError *error, NSString *requestResultJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_sign_request(handle,
            walletHandle,
            [submitterDid UTF8String],
            [requestJson UTF8String],
            IndyWrapperCommon3PSCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)submitRequest:(NSString *)requestJSON
           poolHandle:(IndyHandle)poolHandle
           completion:(void (^)(NSError *error, NSString *requestResultJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_submit_request(handle,
            poolHandle,
            [requestJSON UTF8String],
            IndyWrapperCommon3PSCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

// MARK: - Nym request

+ (void)buildNymRequestWithSubmitterDid:(NSString *)submitterDid
                              targetDID:(NSString *)targetDid
                                 verkey:(NSString *)verkey
                                  alias:(NSString *)alias
                                   role:(NSString *)role
                             completion:(void (^)(NSError *error, NSString *requestJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];


    ret = indy_build_nym_request(handle,
            [submitterDid UTF8String],
            [targetDid UTF8String],
            [verkey UTF8String],
            [alias UTF8String],
            [role UTF8String],
            IndyWrapperCommon3PSCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)buildGetNymRequestWithSubmitterDid:(NSString *)submitterDid
                                 targetDID:(NSString *)targetDid
                                completion:(void (^)(NSError *error, NSString *requestJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_get_nym_request(handle,
            [submitterDid UTF8String],
            [targetDid UTF8String],
            IndyWrapperCommon3PSCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

// MARK: - Attribute request

+ (void)buildAttribRequestWithSubmitterDid:(NSString *)submitterDid
                                 targetDID:(NSString *)targetDid
                                      hash:(NSString *)hash
                                       raw:(NSString *)raw
                                       enc:(NSString *)enc
                                completion:(void (^)(NSError *error, NSString *requestJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_attrib_request(handle,
            [submitterDid UTF8String],
            [targetDid UTF8String],
            [hash UTF8String],
            [raw UTF8String],
            [enc UTF8String],
            IndyWrapperCommon3PSCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)buildGetAttribRequestWithSubmitterDid:(NSString *)submitterDid
                                    targetDID:(NSString *)targetDid
                                          raw:(NSString *)raw
                                         hash:(NSString *)hash
                                          enc:(NSString *)enc
                                   completion:(void (^)(NSError *error, NSString *requestJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_get_attrib_request(handle,
            [submitterDid UTF8String],
            [targetDid UTF8String],
            [raw UTF8String],
            [hash UTF8String],
            [enc UTF8String],
            IndyWrapperCommon3PSCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

// MARK: - Schema request

+ (void)buildSchemaRequestWithSubmitterDid:(NSString *)submitterDid
                                      data:(NSString *)data
                                completion:(void (^)(NSError *error, NSString *requestJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_schema_request(handle,
            [submitterDid UTF8String],
            [data UTF8String],
            IndyWrapperCommon3PSCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)buildGetSchemaRequestWithSubmitterDid:(NSString *)submitterDid
                                         dest:(NSString *)dest
                                         data:(NSString *)data
                                   completion:(void (^)(NSError *error, NSString *requestJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];


    ret = indy_build_get_schema_request(handle,
            [submitterDid UTF8String],
            [dest UTF8String],
            [data UTF8String],
            IndyWrapperCommon3PSCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

// MARK: - ClaimDefTxn request

+ (void)buildClaimDefTxnWithSubmitterDid:(NSString *)submitterDid
                                    xref:(NSNumber *)xref
                           signatureType:(NSString *)signatureType
                                    data:(NSString *)data
                              completion:(void (^)(NSError *error, NSString *requestJSON))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_claim_def_txn(handle,
            [submitterDid UTF8String],
            [xref intValue],
            [signatureType UTF8String],
            [data UTF8String],
            IndyWrapperCommon3PSCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}


+ (void)buildGetClaimDefTxnWithSubmitterDid:(NSString *)submitterDid
                                       xref:(NSNumber *)xref
                              signatureType:(NSString *)signatureType
                                     origin:(NSString *)origin
                                 completion:(void (^)(NSError *error, NSString *requestJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_get_claim_def_txn(handle,
            [submitterDid UTF8String],
            [xref intValue],
            [signatureType UTF8String],
            [origin UTF8String],
            IndyWrapperCommon3PSCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

// MARK: - Ddo request

+ (void)buildGetDdoRequestWithSubmitterDid:(NSString *)submitterDid
                                 targetDID:(NSString *)targetDid
                                completion:(void (^)(NSError *error, NSString *requestResultJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_get_ddo_request(handle,
            [submitterDid UTF8String],
            [targetDid UTF8String],
            IndyWrapperCommon3PSCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

// MARK: - Node request

+ (void)buildNodeRequestWithSubmitterDid:(NSString *)submitterDid
                               targetDid:(NSString *)targetDid
                                    data:(NSString *)data
                              completion:(void (^)(NSError *error, NSString *requestJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_node_request(handle,
            [submitterDid UTF8String],
            [targetDid UTF8String],
            [data UTF8String],
            IndyWrapperCommon3PSCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

// MARK: - Txn request

+ (void)buildGetTxnRequestWithSubmitterDid:(NSString *)submitterDid
                                      data:(NSNumber *)data
                                completion:(void (^)(NSError *error, NSString *requestJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_get_txn_request(handle,
            [submitterDid UTF8String],
            [data intValue],
            IndyWrapperCommon3PSCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

// MARK: - Pool config request

+ (void)buildPoolConfigRequestWithSubmitterDid:(NSString *)submitterDid
                                        writes:(BOOL)writes
                                         force:(BOOL)force
                                    completion:(void (^)(NSError *error, NSString *requestJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_pool_config_request(handle,
            [submitterDid UTF8String],
            (indy_bool_t) writes,
            (indy_bool_t) force,
            IndyWrapperCommon3PSCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

// MARK: - Pool upgrade request

+ (void)buildPoolUpgradeRequestWithSubmitterDid:(NSString *)submitterDid
                                           name:(NSString *)name
                                        version:(NSString *)version
                                         action:(NSString *)action
                                         sha256:(NSString *)sha256
                                        timeout:(NSNumber *)timeout
                                       schedule:(NSString *)schedule
                                  justification:(NSString *)justification
                                      reinstall:(BOOL)reinstall
                                          force:(BOOL)force
                                     completion:(void (^)(NSError *error, NSString *requestJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_pool_upgrade_request(handle,
            [submitterDid UTF8String],
            [name UTF8String],
            [version UTF8String],
            [action UTF8String],
            [sha256 UTF8String],
            [timeout intValue],
            [schedule UTF8String],
            [justification UTF8String],
            (indy_bool_t) reinstall,
            (indy_bool_t) force,
            IndyWrapperCommon3PSCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

@end
