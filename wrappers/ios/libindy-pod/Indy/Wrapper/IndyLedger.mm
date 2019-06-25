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
            IndyWrapperCommonStringCallback);

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
            IndyWrapperCommonStringCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)multiSignRequest:(NSString *)requestJson
            submitterDid:(NSString *)submitterDid
            walletHandle:(IndyHandle)walletHandle
              completion:(void (^)(NSError *error, NSString *requestResultJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_multi_sign_request(handle,
            walletHandle,
            [submitterDid UTF8String],
            [requestJson UTF8String],
            IndyWrapperCommonStringCallback);

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
            IndyWrapperCommonStringCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)submitAction:(NSString *)requestJson
               nodes:(NSString *)nodes
             timeout:(NSNumber *)timeout
          poolHandle:(IndyHandle)poolHandle
          completion:(void (^)(NSError *error, NSString *requestResultJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_submit_action(handle,
            poolHandle,
            [requestJson UTF8String],
            [nodes UTF8String],
            timeout ? [timeout intValue] : -1,
            IndyWrapperCommonStringCallback);

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
            IndyWrapperCommonStringCallback);
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
            IndyWrapperCommonStringCallback);

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
            IndyWrapperCommonStringCallback);

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
            IndyWrapperCommonStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

// MARK: - Get validator info request

+ (void)buildGetValidatorInfo:(NSString *)submitterDid
                   completion:(void (^)(NSError *error, NSString *requestJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];


    ret = indy_build_get_validator_info_request(handle,
            [submitterDid UTF8String],
            IndyWrapperCommonStringCallback);
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
            IndyWrapperCommonStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)buildGetSchemaRequestWithSubmitterDid:(NSString *)submitterDid
                                           id:(NSString *)id
                                   completion:(void (^)(NSError *error, NSString *requestJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];


    ret = indy_build_get_schema_request(handle,
            [submitterDid UTF8String],
            [id UTF8String],
            IndyWrapperCommonStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)parseGetSchemaResponse:(NSString *)getSchemaResponse
                    completion:(void (^)(NSError *error, NSString *schemaId, NSString *schemaJson))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];


    ret = indy_parse_get_schema_response(handle,
            [getSchemaResponse UTF8String],
            IndyWrapperCommonStringStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil);
        });
    }
}

// MARK: - CredDefRequest request

+ (void)buildCredDefRequestWithSubmitterDid:(NSString *)submitterDid
                                       data:(NSString *)data
                                 completion:(void (^)(NSError *error, NSString *requestJSON))completion; {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_cred_def_request(handle,
            [submitterDid UTF8String],
            [data UTF8String],
            IndyWrapperCommonStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)buildGetCredDefRequestWithSubmitterDid:(NSString *)submitterDid
                                            id:(NSString *)id
                                    completion:(void (^)(NSError *error, NSString *requestJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_get_cred_def_request(handle,
            [submitterDid UTF8String],
            [id UTF8String],
            IndyWrapperCommonStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)parseGetCredDefResponse:(NSString *)getCredDefResponse
                     completion:(void (^)(NSError *error, NSString *credDefId, NSString *credDefJson))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];


    ret = indy_parse_get_cred_def_response(handle,
            [getCredDefResponse UTF8String],
            IndyWrapperCommonStringStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil);
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
            IndyWrapperCommonStringCallback);

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
            IndyWrapperCommonStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

// MARK: - Txn request

+ (void)buildGetTxnRequestWithSubmitterDid:(NSString *)submitterDid
                                ledgerType:(NSString *)ledgerType
                                      data:(NSNumber *)data
                                completion:(void (^)(NSError *error, NSString *requestJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_get_txn_request(handle,
            [submitterDid UTF8String],
            [ledgerType UTF8String],
            [data intValue],
            IndyWrapperCommonStringCallback);
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
            IndyWrapperCommonStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

// MARK: - Pool restart request

+ (void)buildPoolRestartRequestWithSubmitterDid:(NSString *)submitterDid
                                         action:(NSString *)action
                                       datetime:(NSString *)datetime
                                     completion:(void (^)(NSError *error, NSString *requestJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_pool_restart_request(handle,
            [submitterDid UTF8String],
            [action UTF8String],
            [datetime UTF8String],
            IndyWrapperCommonStringCallback);
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
                                       package_:(NSString *)package_
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
            [package_ UTF8String],
            IndyWrapperCommonStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

// MARK: - Revocation registry definition request

+ (void)buildRevocRegDefRequestWithSubmitterDid:(NSString *)submitterDid
                                           data:(NSString *)data
                                     completion:(void (^)(NSError *error, NSString *requestJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_revoc_reg_def_request(handle,
            [submitterDid UTF8String],
            [data UTF8String],
            IndyWrapperCommonStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)buildGetRevocRegDefRequestWithSubmitterDid:(NSString *)submitterDid
                                                id:(NSString *)id
                                        completion:(void (^)(NSError *error, NSString *requestJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_get_revoc_reg_def_request(handle,
            [submitterDid UTF8String],
            [id UTF8String],
            IndyWrapperCommonStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)parseGetRevocRegDefResponse:(NSString *)getRevocRegDefResponse
                         completion:(void (^)(NSError *error, NSString *revocRegDefId, NSString *revocRegDefJson))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];


    ret = indy_parse_get_revoc_reg_def_response(handle,
            [getRevocRegDefResponse UTF8String],
            IndyWrapperCommonStringStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil);
        });
    }
}

// MARK: - Revocation registry entry request

+ (void)buildRevocRegEntryRequestWithSubmitterDid:(NSString *)submitterDid
                                             type:(NSString *)type
                                    revocRegDefId:(NSString *)revocRegDefId
                                            value:(NSString *)value
                                       completion:(void (^)(NSError *error, NSString *requestJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_revoc_reg_entry_request(handle,
            [submitterDid UTF8String],
            [revocRegDefId UTF8String],
            [type UTF8String],
            [value UTF8String],
            IndyWrapperCommonStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)buildGetRevocRegRequestWithSubmitterDid:(NSString *)submitterDid
                                  revocRegDefId:(NSString *)revocRegDefId
                                      timestamp:(NSNumber *)timestamp
                                     completion:(void (^)(NSError *error, NSString *requestJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_get_revoc_reg_request(handle,
            [submitterDid UTF8String],
            [revocRegDefId UTF8String],
            [timestamp intValue],
            IndyWrapperCommonStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)parseGetRevocRegResponse:(NSString *)getRevocRegResponse
                      completion:(void (^)(NSError *error, NSString *revocRegDefId, NSString *revocRegJson, NSNumber *timestamp))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_parse_get_revoc_reg_response(handle,
            [getRevocRegResponse UTF8String],
            IndyWrapperCommonStringStringLongCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil, nil);
        });
    }
}

+ (void)buildGetRevocRegDeltaRequestWithSubmitterDid:(NSString *)submitterDid
                                       revocRegDefId:(NSString *)revocRegDefId
                                                from:(NSNumber *)from
                                                  to:(NSNumber *)to
                                          completion:(void (^)(NSError *error, NSString *requestJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_get_revoc_reg_delta_request(handle,
            [submitterDid UTF8String],
            [revocRegDefId UTF8String],
            [from intValue],
            [to intValue],
            IndyWrapperCommonStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)parseGetRevocRegDeltaResponse:(NSString *)getRevocRegDeltaResponse
                           completion:(void (^)(NSError *error, NSString *revocRegDefId, NSString *revocRegDeltaJson, NSNumber *timestamp))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_parse_get_revoc_reg_delta_response(handle,
            [getRevocRegDeltaResponse UTF8String],
            IndyWrapperCommonStringStringLongCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil, nil);
        });
    }
}

+ (void)buildAuthRuleRequestWithSubmitterDid:(NSString *)submitterDid
                                     txnType:(NSString *)txnType
                                      action:(NSString *)action
                                       field:(NSString *)field
                                    oldValue:(NSString *)oldValue
                                    newValue:(NSString *)newValue
                                  constraint:(NSString *)constraint
                                  completion:(void (^)(NSError *error, NSString *requestJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];


    ret = indy_build_auth_rule_request(handle,
            [submitterDid UTF8String],
            [txnType UTF8String],
            [action UTF8String],
            [field UTF8String],
            [oldValue UTF8String],
            [newValue UTF8String],
            [constraint UTF8String],
            IndyWrapperCommonStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)buildAuthRulesRequestWithSubmitterDid:(NSString *)submitterDid
                                         data:(NSString *)data
                                  completion:(void (^)(NSError *error, NSString *requestJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];


    ret = indy_build_auth_rules_request(handle,
            [submitterDid UTF8String],
            [data UTF8String],
            IndyWrapperCommonStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)buildGetAuthRuleRequestWithSubmitterDid:(NSString *)submitterDid
                                        txnType:(NSString *)txnType
                                         action:(NSString *)action
                                          field:(NSString *)field
                                       oldValue:(NSString *)oldValue
                                       newValue:(NSString *)newValue
                                     completion:(void (^)(NSError *error, NSString *requestJSON))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];


    ret = indy_build_get_auth_rule_request(handle,
            [submitterDid UTF8String],
            [txnType UTF8String],
            [action UTF8String],
            [field UTF8String],
            [oldValue UTF8String],
            [newValue UTF8String],
            IndyWrapperCommonStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)getResponseMetadata:(NSString *)response
                 completion:(void (^)(NSError *error, NSString *responseMetadata))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_get_response_metadata(handle,
            [response UTF8String],
            IndyWrapperCommonStringCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)buildTxnAuthorAgreementRequestWithSubmitterDid:(NSString *)submitterDid
                                                  text:(NSString *)text
                                               version:(NSString *)version
                                            completion:(void (^)(NSError *error, NSString *responseMetadata))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_txn_author_agreement_request(handle,
            [submitterDid UTF8String],
            [text UTF8String],
            [version UTF8String],
            IndyWrapperCommonStringCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)buildGetTxnAuthorAgreementRequestWithSubmitterDid:(NSString *)submitterDid
                                                     data:(NSString *)data
                                               completion:(void (^)(NSError *error, NSString *responseMetadata))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_get_txn_author_agreement_request(handle,
            [submitterDid UTF8String],
            [data UTF8String],
            IndyWrapperCommonStringCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)buildAcceptanceMechanismsRequestWithSubmitterDid:(NSString *)submitterDid
                                                     aml:(NSString *)aml
                                                 version:(NSString *)version
                                              amlContext:(NSString *)amlContext
                                              completion:(void (^)(NSError *error, NSString *responseMetadata))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_acceptance_mechanisms_request(handle,
            [submitterDid UTF8String],
            [aml UTF8String],
            [version UTF8String],
            [amlContext UTF8String],
            IndyWrapperCommonStringCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)buildGetAcceptanceMechanismsRequestWithSubmitterDid:(NSString *)submitterDid
                                                  timestamp:(NSNumber *)timestamp
                                                    version:(NSString *)version
                                                 completion:(void (^)(NSError *error, NSString *responseMetadata))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_get_acceptance_mechanisms_request(handle,
            [submitterDid UTF8String],
            timestamp ? [timestamp longLongValue] : -1,
            [version UTF8String],
            IndyWrapperCommonStringCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)appendTxnAuthorAgreementAcceptanceToRequest:(NSString *)requestJson
                                               text:(NSString *)text
                                            version:(NSString *)version
                                          taaDigest:(NSString *)taaDigest
                                        accMechType:(NSString *)accMechType
                                   timeOfAcceptance:(NSNumber *)timeOfAcceptance
                                         completion:(void (^)(NSError *error, NSString *responseMetadata))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_append_txn_author_agreement_acceptance_to_request(handle,
            [requestJson UTF8String],
            [text UTF8String],
            [version UTF8String],
            [taaDigest UTF8String],
            [accMechType UTF8String],
            [timeOfAcceptance unsignedLongLongValue],
            IndyWrapperCommonStringCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

@end
