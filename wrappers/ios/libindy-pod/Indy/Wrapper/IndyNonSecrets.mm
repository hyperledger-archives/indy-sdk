//
// Created by Evernym on 5/16/18.
// Copyright (c) 2018 Hyperledger. All rights reserved.
//

#import "IndyCallbacks.h"
#import "NSError+IndyError.h"
#import "IndyNonSecrets.h"


@implementation IndyNonSecrets

+ (void)addRecordInWallet:(IndyHandle)walletHandle
                     type:(NSString *)type
                       id:(NSString *)id
                    value:(NSString *)value
                 tagsJson:(NSString *)tagsJson
               completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    [completion copy];
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_add_wallet_record(handle,
            walletHandle,
            [type UTF8String],
            [id UTF8String],
            [value UTF8String],
            [tagsJson UTF8String],
            IndyWrapperCommonCallback
    );
    [[IndyCallbacks sharedInstance] complete:completion forHandle:handle ifError:ret];
}

+ (void)updateRecordValueInWallet:(IndyHandle)walletHandle
                             type:(NSString *)type
                               id:(NSString *)id
                            value:(NSString *)value
                       completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    [completion copy];
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_update_wallet_record_value(handle,
            walletHandle,
            [type UTF8String],
            [id UTF8String],
            [value UTF8String],
            IndyWrapperCommonCallback
    );
    [[IndyCallbacks sharedInstance] complete:completion forHandle:handle ifError:ret];
}

+ (void)updateRecordTagsInWallet:(IndyHandle)walletHandle
                            type:(NSString *)type
                              id:(NSString *)id
                        tagsJson:(NSString *)tagsJson
                      completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    [completion copy];
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_update_wallet_record_tags(handle,
            walletHandle,
            [type UTF8String],
            [id UTF8String],
            [tagsJson UTF8String],
            IndyWrapperCommonCallback
    );
    [[IndyCallbacks sharedInstance] complete:completion forHandle:handle ifError:ret];
}

+ (void)addRecordTagsInWallet:(IndyHandle)walletHandle
                         type:(NSString *)type
                           id:(NSString *)id
                     tagsJson:(NSString *)tagsJson
                   completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    [completion copy];
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_add_wallet_record_tags(handle,
            walletHandle,
            [type UTF8String],
            [id UTF8String],
            [tagsJson UTF8String],
            IndyWrapperCommonCallback
    );
    [[IndyCallbacks sharedInstance] complete:completion forHandle:handle ifError:ret];
}

+ (void)deleteRecordTagsInWallet:(IndyHandle)walletHandle
                            type:(NSString *)type
                              id:(NSString *)id
                       tagsNames:(NSString *)tagsNames
                      completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    [completion copy];
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_delete_wallet_record_tags(handle,
            walletHandle,
            [type UTF8String],
            [id UTF8String],
            [tagsNames UTF8String],
            IndyWrapperCommonCallback
    );
    [[IndyCallbacks sharedInstance] complete:completion forHandle:handle ifError:ret];
}

+ (void)deleteRecordInWallet:(IndyHandle)walletHandle
                        type:(NSString *)type
                          id:(NSString *)id
                  completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    [completion copy];
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_delete_wallet_record(handle,
            walletHandle,
            [type UTF8String],
            [id UTF8String],
            IndyWrapperCommonCallback
    );
    [[IndyCallbacks sharedInstance] complete:completion forHandle:handle ifError:ret];
}

+ (void)getRecordFromWallet:(IndyHandle)walletHandle
                       type:(NSString *)type
                         id:(NSString *)id
                optionsJson:(NSString *)optionsJson
                 completion:(void (^)(NSError *error, NSString *recordJson))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_get_wallet_record(handle,
            walletHandle,
            [type UTF8String],
            [id UTF8String],
            [optionsJson UTF8String],
            IndyWrapperCommonStringCallback);
    [[IndyCallbacks sharedInstance] completeStr:completion forHandle:handle ifError:ret];
}

+ (void)openSearchInWallet:(IndyHandle)walletHandle
                      type:(NSString *)type
                 queryJson:(NSString *)queryJson
               optionsJson:(NSString *)optionsJson
                completion:(void (^)(NSError *error, IndyHandle searchHandle))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_open_wallet_search(handle,
            walletHandle,
            [type UTF8String],
            [queryJson UTF8String],
            [optionsJson UTF8String],
            IndyWrapperCommonHandleCallback
    );

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], 0);
        });
    }
}

+ (void)fetchNextRecordsFromSearch:(IndyHandle)searchHandle
                      walletHandle:(IndyHandle)walletHandle
                             count:(NSNumber *)count
                        completion:(void (^)(NSError *error, NSString *recordsJson))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_fetch_wallet_search_next_records(handle,
            walletHandle,
            searchHandle,
            [count unsignedIntValue],
            IndyWrapperCommonStringCallback);
    [[IndyCallbacks sharedInstance] completeStr:completion forHandle:handle ifError:ret];
}

+ (void)closeSearchWithHandle:(IndyHandle)searchHandle
                   completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_close_wallet_search(handle,
            searchHandle,
            IndyWrapperCommonCallback
    );

    [[IndyCallbacks sharedInstance] complete:completion forHandle:handle ifError:ret];
}

@end