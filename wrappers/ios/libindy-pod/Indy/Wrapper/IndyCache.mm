//
// Created by Evernym on 5/16/18.
// Copyright (c) 2018 Hyperledger. All rights reserved.
//

#import "IndyCallbacks.h"
#import "NSError+IndyError.h"
#import "IndyCache.h"


@implementation IndyCache

+ (void)getSchema:(IndyHandle)poolHandle
     walletHandle:(IndyHandle)walletHandle
     submitterDid:(NSString *)submitterDid
               id:(NSString *)id
      optionsJson:(NSString *)optionsJson
       completion:(void (^)(NSError *error, NSString *schemaJson))completion {

    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_get_schema(handle,
            poolHandle,
            walletHandle,
            [submitterDid UTF8String],
            [id UTF8String],
            [optionsJson UTF8String],
            IndyWrapperCommonStringCallback);
    [[IndyCallbacks sharedInstance] completeStr:completion forHandle:handle ifError:ret];
}

+ (void)getCredDef:(IndyHandle)poolHandle
      walletHandle:(IndyHandle)walletHandle
      submitterDid:(NSString *)submitterDid
                id:(NSString *)id
       optionsJson:(NSString *)optionsJson
        completion:(void (^)(NSError *error, NSString *credDefJson))completion {

    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_get_cred_def(handle,
            poolHandle,
            walletHandle,
            [submitterDid UTF8String],
            [id UTF8String],
            [optionsJson UTF8String],
            IndyWrapperCommonStringCallback);
    [[IndyCallbacks sharedInstance] completeStr:completion forHandle:handle ifError:ret];
}

+ (void)purgeSchemaCache:(IndyHandle)walletHandle
             optionsJson:(NSString *)optionsJson
              completion:(void (^)(NSError *error))completion {

    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_purge_schema_cache(handle,
            walletHandle,
            [optionsJson UTF8String],
            IndyWrapperCommonCallback);

    [[IndyCallbacks sharedInstance] complete:completion forHandle:handle ifError:ret];
}

+ (void)purgeCredDefCache:(IndyHandle)walletHandle
              optionsJson:(NSString *)optionsJson
               completion:(void (^)(NSError *error))completion {

    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_purge_cred_def_cache(handle,
            walletHandle,
            [optionsJson UTF8String],
            IndyWrapperCommonCallback);

    [[IndyCallbacks sharedInstance] complete:completion forHandle:handle ifError:ret];
}

@end
