//
//  IndyWallet.m
//  libindy
//


#import "IndyWalletCallbacks.h"
#import "IndyWallet.h"
#import "IndyCallbacks.h"
#import "indy_core.h"
#import "NSError+IndyError.h"
#import "IndyKeychainWallet.h"


@implementation IndyWallet

+ (IndyWallet *)sharedInstance {
    static IndyWallet *instance = nil;
    static dispatch_once_t dispatch_once_block;

    dispatch_once(&dispatch_once_block, ^{
        instance = [IndyWallet new];
    });

    return instance;
}


- (void)registerWalletType:(NSString *)type
        withImplementation:(Class <IndyWalletProtocol>)implementation
                completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    BOOL addedType = [[IndyWalletCallbacks sharedInstance] setupCustomWalletImplementation:implementation];

//    if (addedType == NO)
//    {
//        // custom wallet implementation is already registered
//        dispatch_async(dispatch_get_main_queue(), ^{
//            completion([NSError errorFromIndyError: WalletTypeAlreadyRegisteredError]);
//        });
//    }
//
//    ret = indy_register_wallet_type(handle,
//            [type UTF8String],
//            CustomWalletCreateCallback,
//            CustomWalletOpenCallback,
//            CustomWalletSetCallback,
//            CustomWalletGetCallback,
//            CustomWalletGetNotExpiredCallback,
//            CustomWalletListCallback,
//            CustomWalletCloseCallback,
//            CustomWalletDeleteCallback,
//            CustomWalletFreeCallback,
//            IndyWrapperCommonCallback);
//
    if (ret != Success) {
        [[IndyWalletCallbacks sharedInstance] removeCustomWalletImplementation];
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret]);
        });
    }
}

- (void)registerIndyKeychainWalletType:(NSString *)type
                            completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

//    ret = indy_register_wallet_type(handle,
//            [type UTF8String],
//            IndyKeychainWalletCreateCallback,
//            IndyKeychainWalletOpenCallback,
//            IndyKeychainWalletSetCallback,
//            IndyKeychainWalletGetCallback,
//            IndyKeychainWalletGetNotExpiredCallback,
//            IndyKeychainWalletListCallback,
//            IndyKeychainWalletCloseCallback,
//            IndyKeychainWalletDeleteCallback,
//            IndyKeychainWalletFreeCallback,
//            IndyWrapperCommonCallback);
//
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret]);
        });
    }
}

- (void)createWalletWithConfig:(NSString *)config
                 credentials:(NSString *)credentials
                  completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    [completion copy];
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_create_wallet(handle,
            [config UTF8String],
            [credentials UTF8String],
            IndyWrapperCommonCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret]);
        });
    }
}

- (void)openWalletWithConfig:(NSString *)config
               credentials:(NSString *)credentials
                completion:(void (^)(NSError *error, IndyHandle walletHandle))completion {
    indy_error_t ret;

    //id hghg = [completion copy];
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_open_wallet(handle,
            [config UTF8String],
            [credentials UTF8String],
            IndyWrapperCommonHandleCallback
    );

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], 0);
        });
    }
}

- (void)closeWalletWithHandle:(IndyHandle)walletHandle
                   completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_close_wallet(handle,
            walletHandle,
            IndyWrapperCommonCallback
    );

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret]);
        });
    }
}

- (void)deleteWalletWithConfig:(NSString *)config
                 credentials:(NSString *)credentials
                  completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_delete_wallet(handle,
            [config UTF8String],
            [credentials UTF8String],
            IndyWrapperCommonCallback
    );

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret]);
        });
    }
}


- (void)exportWalletWithHandle:(IndyHandle)walletHandle
              exportConfigJson:(NSString *)exportConfigJson
                    completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_export_wallet(handle,
            walletHandle,
            [exportConfigJson UTF8String],
            IndyWrapperCommonCallback
    );

    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret]);
        });
    }
}

- (void)importWalletWithConfig:(NSString *)config
                 credentials:(NSString *)credentials
            importConfigJson:(NSString *)importConfigJson
                  completion:(void (^)(NSError *error))completion {
    indy_error_t ret;

    [completion copy];
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_import_wallet(handle,
            [config UTF8String],
            [credentials UTF8String],
            [importConfigJson UTF8String],
            IndyWrapperCommonCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret]);
        });
    }
}

- (void)cleanupIndyKeychainWallet {
    [[IndyKeychainWallet sharedInstance] cleanup];
}

@end
