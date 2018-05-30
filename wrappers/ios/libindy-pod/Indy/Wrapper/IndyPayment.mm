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
              completion:(void (^)(NSError *error, NSString *requestWithFeesJson, NSString *paymentMethod))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];


    ret = indy_add_request_fees(handle,
            walletHandle,
            [submitterDid UTF8String],
            [requestJson UTF8String],
            [inputsJson UTF8String],
            [outputsJson UTF8String],
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
                   completion:(void (^)(NSError *error, NSString *utxoJson))completion {
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

+ (void)buildGetUtxoRequest:(IndyHandle)walletHandle
               submitterDid:(NSString *)submitterDid
             paymentAddress:(NSString *)paymentAddress
                 completion:(void (^)(NSError *error, NSString *getUtxoTxnJson, NSString *paymentMethod))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];


    ret = indy_build_get_utxo_request(handle,
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

+ (void)parseGetUtxoResponse:(NSString *)responseJson
               paymentMethod:(NSString *)paymentMethod
                  completion:(void (^)(NSError *error, NSString *utxoJson))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_parse_get_utxo_response(handle,
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


+ (void)buildPaymentRequest:(IndyHandle)walletHandle
               submitterDid:(NSString *)submitterDid
                 inputsJson:(NSString *)inputsJson
                outputsJson:(NSString *)outputsJson
                 completion:(void (^)(NSError *error, NSString *paymentReqJson, NSString *paymentMethod))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];


    ret = indy_build_payment_req(handle,
            walletHandle,
            [submitterDid UTF8String],
            [inputsJson UTF8String],
            [outputsJson UTF8String],
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
                  completion:(void (^)(NSError *error, NSString *utxoJson))completion {
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

+ (void)buildMintRequest:(IndyHandle)walletHandle
            submitterDid:(NSString *)submitterDid
             outputsJson:(NSString *)outputsJson
              completion:(void (^)(NSError *error, NSString *mintReqJson, NSString *paymentMethod))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];


    ret = indy_build_mint_req(handle,
            walletHandle,
            [submitterDid UTF8String],
            [outputsJson UTF8String],
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


@end