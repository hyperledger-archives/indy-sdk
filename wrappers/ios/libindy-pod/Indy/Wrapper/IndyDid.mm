//
//  IndyDid.m
//  libindy
//

#import <libindy/indy_types.h>
#import "IndyDid.h"
#import "indy_mod.h"
#import "IndyCallbacks.h"

@implementation IndyDid

+ (void)createAndStoreMyDid:(NSString *)didJson
               walletHandle:(IndyHandle)walletHandle
                 completion:(void (^)(NSError *error,
                         NSString *did,
                         NSString *verkey))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_create_and_store_my_did(handle,
            walletHandle,
            [didJson UTF8String],
            IndyWrapperCommonStringStringCallback);
    [[IndyCallbacks sharedInstance] complete2Str:completion forHandle:handle ifError:ret];
}

+ (void)replaceKeysStartForDid:(NSString *)did
                  identityJson:(NSString *)identityJson
                  walletHandle:(IndyHandle)walletHandle
                    completion:(void (^)(NSError *error,
                            NSString *verkey))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_replace_keys_start(handle,
            walletHandle,
            [did UTF8String],
            [identityJson UTF8String],
            IndyWrapperCommonStringCallback);
    [[IndyCallbacks sharedInstance] completeStr:completion forHandle:handle ifError:ret];
}

+ (void)replaceKeysApplyForDid:(NSString *)did
                  walletHandle:(IndyHandle)walletHandle
                    completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_replace_keys_apply(handle,
            walletHandle,
            [did UTF8String],
            IndyWrapperCommonCallback);
    [[IndyCallbacks sharedInstance] complete:completion forHandle:handle ifError:ret];
}

+ (void)storeTheirDid:(NSString *)identityJSON
         walletHandle:(IndyHandle)walletHandle
           completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_store_their_did(handle,
            walletHandle,
            [identityJSON UTF8String],
            IndyWrapperCommonCallback
    );
    [[IndyCallbacks sharedInstance] complete:completion forHandle:handle ifError:ret];
}

+ (void)keyForDid:(NSString *)did
       poolHandle:(IndyHandle)poolHandle
     walletHandle:(IndyHandle)walletHandle
       completion:(void (^)(NSError *error, NSString *key))completion {
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    indy_error_t ret = indy_key_for_did(handle, poolHandle, walletHandle, [did UTF8String], IndyWrapperCommonStringCallback);

    [[IndyCallbacks sharedInstance] completeStr:completion forHandle:handle ifError:ret];
}

+ (void)keyForLocalDid:(NSString *)did
          walletHandle:(IndyHandle)walletHandle
            completion:(void (^)(NSError *error, NSString *key))completion {
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    indy_error_t ret = indy_key_for_local_did(handle, walletHandle, [did UTF8String], IndyWrapperCommonStringCallback);

    [[IndyCallbacks sharedInstance] completeStr:completion forHandle:handle ifError:ret];
}

+ (void)setEndpointAddress:(NSString *)address
              transportKey:(NSString *)transportKey
                    forDid:(NSString *)did
              walletHandle:(IndyHandle)walletHandle
                completion:(void (^)(NSError *error))completion {
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    indy_error_t ret = indy_set_endpoint_for_did(handle,
            walletHandle,
            [did UTF8String],
            [address UTF8String],
            [transportKey UTF8String],
            IndyWrapperCommonCallback);

    [[IndyCallbacks sharedInstance] complete:completion forHandle:handle ifError:ret];
}

+ (void)getEndpointForDid:(NSString *)did
             walletHandle:(IndyHandle)walletHandle
               poolHandle:(IndyHandle)poolHandle
               completion:(void (^)(NSError *error, NSString *address, NSString *transportKey))completion {
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    indy_error_t ret = indy_get_endpoint_for_did(
            handle,
            walletHandle,
            poolHandle,
            [did UTF8String],
            IndyWrapperCommonStringOptStringCallback);

    [[IndyCallbacks sharedInstance] complete2Str:completion forHandle:handle ifError:ret];
}

+ (void)setMetadata:(NSString *)metadata
             forDid:(NSString *)did
       walletHandle:(IndyHandle)walletHandle
         completion:(void (^)(NSError *error))completion {
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    indy_error_t ret = indy_set_did_metadata(handle,
            walletHandle,
            [did UTF8String],
            [metadata UTF8String],
            IndyWrapperCommonCallback);

    [[IndyCallbacks sharedInstance] complete:completion forHandle:handle ifError:ret];
}

+ (void)getMetadataForDid:(NSString *)did
             walletHandle:(IndyHandle)walletHandle
               completion:(void (^)(NSError *error, NSString *metadata))completion {
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    indy_error_t ret = indy_get_did_metadata(handle,
            walletHandle,
            [did UTF8String],
            IndyWrapperCommonStringCallback);

    [[IndyCallbacks sharedInstance] completeStr:completion forHandle:handle ifError:ret];
}

+ (void)abbreviateVerkey:(NSString *)did
              fullVerkey:(NSString *)fullVerkey
              completion:(void (^)(NSError *error, NSString *verkey))completion {
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    indy_error_t ret = indy_abbreviate_verkey(handle, [did UTF8String], [fullVerkey UTF8String], IndyWrapperCommonStringCallback);

    [[IndyCallbacks sharedInstance] completeStr:completion forHandle:handle ifError:ret];
}

+ (void)listMyDidsWithMeta:(IndyHandle)walletHandle
                completion:(void (^)(NSError *error, NSString *metadata))completion {
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    indy_error_t ret = indy_list_my_dids_with_meta(handle, walletHandle, IndyWrapperCommonStringCallback);
    [[IndyCallbacks sharedInstance] completeStr:completion forHandle:handle ifError:ret];
}


@end
