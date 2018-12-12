#import <Foundation/Foundation.h>
#import "IndyCallbacks.h"
#import "IndyPool.h"
#import "indy_core.h"
#import "NSError+IndyError.h"#import "indy_mod.h"

@implementation IndyPool

+ (void)createPoolLedgerConfigWithPoolName:(NSString *)name
                                poolConfig:(NSString *)poolConfig
                                completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_create_pool_ledger_config(handle,
            [name UTF8String],
            [poolConfig UTF8String],
            IndyWrapperCommonCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret]);
        });
    }
}

+ (void)openPoolLedgerWithName:(NSString *)name
                    poolConfig:(NSString *)poolConfig
                    completion:(void (^)(NSError *error, IndyHandle poolHandle))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_open_pool_ledger(handle,
            [name UTF8String],
            [poolConfig UTF8String],
            IndyWrapperCommonHandleCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], 0);
        });
    }
}

+ (void)refreshPoolLedgerWithHandle:(IndyHandle)poolHandle
                         completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_refresh_pool_ledger(handle,
            (indy_handle_t) poolHandle,
            IndyWrapperCommonCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret]);
        });
    }
}

+ (void)closePoolLedgerWithHandle:(IndyHandle)poolHandle
                       completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_close_pool_ledger(handle,
            (indy_handle_t) poolHandle,
            IndyWrapperCommonCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret]);
        });
    }
}

+ (void)deletePoolLedgerConfigWithName:(NSString *)name
                            completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_delete_pool_ledger_config(handle,
            [name UTF8String],
            IndyWrapperCommonCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret]);
        });
    }
}

+ (void)setProtocolVersion:(NSNumber *)protocolVersion
                completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_set_protocol_version(handle,
            [protocolVersion intValue],
            IndyWrapperCommonCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret]);
        });
    }
}

@end

