#import "IndyCallbacks.h"
#import "NSError+IndyError.h"
#import "IndyPayment.h"


@implementation IndyPayment

+ (void)createPaymentAddressForMethod:(NSString *)paymentMethod
                         walletHandle:(IndyHandle)walletHandle
                               config:(NSString *)config
                           completion:(void (^)(NSError *error, NSString *paymentAddress))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_create_payment_address(handle,
            walletHandle,
            [paymentMethod UTF8String],
            [config UTF8String],
            IndyWrapperCommonStringCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)listPaymentAddresses:(IndyHandle)walletHandle
                  completion:(void (^)(NSError *error, NSString *paymentAddressesJson))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_list_payment_addresses(handle,
            walletHandle,
            IndyWrapperCommonStringCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)addFeesToRequest:(NSString *)requestJson
            walletHandle:(IndyHandle)walletHandle
            submitterDid:(NSString *)submitterDid
              inputsJson:(NSString *)inputsJson
             outputsJson:(NSString *)outputsJson
                   extra:(NSString *)extra
              completion:(void (^)(NSError *error, NSString *requestWithFeesJson, NSString *paymentMethod))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];


    ret = indy_add_request_fees(handle,
            walletHandle,
            [submitterDid UTF8String],
            [requestJson UTF8String],
            [inputsJson UTF8String],
            [outputsJson UTF8String],
            [extra UTF8String],
            IndyWrapperCommonStringStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil);
        });
    }
}

+ (void)parseResponseWithFees:(NSString *)responseJson
                paymentMethod:(NSString *)paymentMethod
                   completion:(void (^)(NSError *error, NSString *receiptsJson))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_parse_response_with_fees(handle,
            [paymentMethod UTF8String],
            [responseJson UTF8String],
            IndyWrapperCommonStringCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)buildGetPaymentSourcesRequest:(IndyHandle)walletHandle
                         submitterDid:(NSString *)submitterDid
                       paymentAddress:(NSString *)paymentAddress
                           completion:(void (^)(NSError *error, NSString *getSourcesTxnJson, NSString *paymentMethod))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];


    ret = indy_build_get_payment_sources_request(handle,
            walletHandle,
            [submitterDid UTF8String],
            [paymentAddress UTF8String],
            IndyWrapperCommonStringStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil);
        });
    }
}

+ (void)buildGetPaymentSourcesWithFromRequest:(IndyHandle)walletHandle
                                 submitterDid:(NSString *)submitterDid
                               paymentAddress:(NSString *)paymentAddress
                                         from:(NSNumber *)from
                                   completion:(void (^)(NSError *error, NSString *getSourcesTxnJson, NSString *paymentMethod))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];


    ret = indy_build_get_payment_sources_with_from_request(handle,
            walletHandle,
            [submitterDid UTF8String],
            [paymentAddress UTF8String],
            [from intValue],
            IndyWrapperCommonStringStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil);
        });
    }
}

+ (void)parseGetPaymentSourcesResponse:(NSString *)responseJson
                         paymentMethod:(NSString *)paymentMethod
                            completion:(void (^)(NSError *error, NSString *sourcesJson))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_parse_get_payment_sources_response(handle,
            [paymentMethod UTF8String],
            [responseJson UTF8String],
            IndyWrapperCommonStringCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)parseGetPaymentSourcesWithFromResponse:(NSString *)responseJson
                                 paymentMethod:(NSString *)paymentMethod
                                    completion:(void (^)(NSError *error, NSString *sourcesJson, NSNumber *next))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_parse_get_payment_sources_with_from_response(handle,
            [paymentMethod UTF8String],
            [responseJson UTF8String],
            IndyWrapperCommonStringNumber64Callback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil);
        });
    }
}

+ (void)buildPaymentRequest:(IndyHandle)walletHandle
               submitterDid:(NSString *)submitterDid
                 inputsJson:(NSString *)inputsJson
                outputsJson:(NSString *)outputsJson
                      extra:(NSString *)extra
                 completion:(void (^)(NSError *error, NSString *paymentReqJson, NSString *paymentMethod))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];


    ret = indy_build_payment_req(handle,
            walletHandle,
            [submitterDid UTF8String],
            [inputsJson UTF8String],
            [outputsJson UTF8String],
            [extra UTF8String],
            IndyWrapperCommonStringStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil);
        });
    }
}

+ (void)parsePaymentResponse:(NSString *)responseJson
               paymentMethod:(NSString *)paymentMethod
                  completion:(void (^)(NSError *error, NSString *receiptsJson))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_parse_payment_response(handle,
            [paymentMethod UTF8String],
            [responseJson UTF8String],
            IndyWrapperCommonStringCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)preparePaymentExtraWithAcceptanceData:(NSString *)extraJson
                                         text:(NSString *)text
                                      version:(NSString *)version
                                    taaDigest:(NSString *)taaDigest
                                  accMechType:(NSString *)accMechType
                             timeOfAcceptance:(NSNumber *)timeOfAcceptance
                                   completion:(void (^)(NSError *error, NSString *extraWithAcceptance))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_prepare_payment_extra_with_acceptance_data(handle,
            [extraJson UTF8String],
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

+ (void)buildMintRequest:(IndyHandle)walletHandle
            submitterDid:(NSString *)submitterDid
             outputsJson:(NSString *)outputsJson
                   extra:(NSString *)extra
              completion:(void (^)(NSError *error, NSString *mintReqJson, NSString *paymentMethod))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];


    ret = indy_build_mint_req(handle,
            walletHandle,
            [submitterDid UTF8String],
            [outputsJson UTF8String],
            [extra UTF8String],
            IndyWrapperCommonStringStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil);
        });
    }
}

+ (void)buildSetTxnFeesRequest:(IndyHandle)walletHandle
                  submitterDid:(NSString *)submitterDid
                 paymentMethod:(NSString *)paymentMethod
                      feesJson:(NSString *)feesJson
                    completion:(void (^)(NSError *error, NSString *setTxnFeesReqJson))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_set_txn_fees_req(handle,
            walletHandle,
            [submitterDid UTF8String],
            [paymentMethod UTF8String],
            [feesJson UTF8String],
            IndyWrapperCommonStringCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)buildGetTxnFeesRequest:(IndyHandle)walletHandle
                  submitterDid:(NSString *)submitterDid
                 paymentMethod:(NSString *)paymentMethod
                    completion:(void (^)(NSError *error, NSString *getTxnFeesReqJson))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_build_get_txn_fees_req(handle,
            walletHandle,
            [submitterDid UTF8String],
            [paymentMethod UTF8String],
            IndyWrapperCommonStringCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)parseGetTxnFeesResponse:(NSString *)responseJson
                  paymentMethod:(NSString *)paymentMethod
                     completion:(void (^)(NSError *error, NSString *feesJson))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_parse_get_txn_fees_response(handle,
            [paymentMethod UTF8String],
            [responseJson UTF8String],
            IndyWrapperCommonStringCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)buildVerifyPaymentRequest:(IndyHandle)walletHandle
                     submitterDid:(NSString *)submitterDid
                          receipt:(NSString *)receipt
                       completion:(void (^)(NSError *error, NSString *verifyReqJson, NSString *paymentMethod))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];


    ret = indy_build_verify_payment_req(handle,
            walletHandle,
            [submitterDid UTF8String],
            [receipt UTF8String],
            IndyWrapperCommonStringStringCallback);
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil);
        });
    }
}

+ (void)parseVerifyPaymentResponse:(NSString *)responseJson
                     paymentMethod:(NSString *)paymentMethod
                        completion:(void (^)(NSError *error, NSString *txnJson))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_parse_verify_payment_response(handle,
            [paymentMethod UTF8String],
            [responseJson UTF8String],
            IndyWrapperCommonStringCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)getRequestInfoForRequester:(NSString *)requesterInfoJson
           getAuthRuleResponseJson:(NSString *)getAuthRuleResponseJson
                          feesJson:(NSString *)feesJson
                        completion:(void (^)(NSError *error, NSString *requestInfoJson))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_get_request_info(handle,
            [getAuthRuleResponseJson UTF8String],
            [requesterInfoJson UTF8String],
            [feesJson UTF8String],
            IndyWrapperCommonStringCallback);

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)signWithAddress:(NSString *)address
                message:(NSData *)message
           walletHandle:(IndyHandle)walletHandle
             completion:(void (^)(NSError *error, NSData *signature))completion
{
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    uint32_t messageLen = (uint32_t) [message length];
    uint8_t *messageRaw = (uint8_t *) [message bytes];
    indy_error_t ret = indy_sign_with_address(handle,
            walletHandle,
            [address UTF8String],
            messageRaw,
            messageLen,
            IndyWrapperCommonDataCallback);

    [[IndyCallbacks sharedInstance] completeData:completion forHandle:handle ifError:ret];
}

+ (void)verifyWithAddress:(NSString *)address
                  message:(NSData *)message
                signature:(NSData *)signature
               completion:(void (^)(NSError *error, BOOL valid))completion
{
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    uint32_t messageLen = (uint32_t) [message length];
    uint8_t *messageRaw = (uint8_t *) [message bytes];
    uint32_t signatureLen = (uint32_t) [signature length];
    uint8_t *signatureRaw = (uint8_t *) [signature bytes];

    indy_error_t ret = indy_verify_with_address(handle,
            [address UTF8String],
            messageRaw,
            messageLen,
            signatureRaw,
            signatureLen,
            IndyWrapperCommonBoolCallback);

    [[IndyCallbacks sharedInstance] completeBool:completion forHandle:handle ifError:ret];
}

@end