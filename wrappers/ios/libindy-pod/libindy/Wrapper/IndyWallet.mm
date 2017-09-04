//
//  IndyWallet.m
//  libindy
//


#import "IndyWalletCallbacks.h"
#import "IndyWallet.h"
#import "IndyCallbacks.h"
#import "indy_core.h"
#import "NSError+IndyError.h"


@implementation IndyWallet

+ (IndyWallet*) sharedInstance
{
    static IndyWallet *instance = nil;
    static dispatch_once_t dispatch_once_block;
    
    dispatch_once(&dispatch_once_block, ^
                  {
                      instance = [IndyWallet new];
                  });
    
    return instance;
}

- (NSError *)registerWalletType:(NSString *)type
             withImplementation:(id<IndyWalletImplementation>)implementation
                     completion:(void (^)(NSError *error)) handler
{
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor: (void*)handler];
    
    [[IndyWalletCallbacks sharedInstance] addWalletType:type
                                     withImplementation:implementation];
    
    ret = indy_register_wallet_type(handle,
                                    [type UTF8String],
                                    IndyWalletCreateCallback,
                                    IndyWalletOpenCallback,
                                    IndyWalletSetCallback,
                                    IndyWalletGetCallback,
                                    IndyWalletGetNotExpiredCallback,
                                    IndyWalletListCallback,
                                    IndyWalletCloseCallback,
                                    IndyWalletDeleteCallback,
                                    IndyWalletFreeCallback,
                                    IndyWrapperCommon2PCallback);
    
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

- (NSError *)createWalletWithPoolName:(NSString *)poolName
                                 name:(NSString *)name
                                xType:(NSString *)type
                               config:(NSString *)config
                          credentials:(NSString *)credentials
                           completion:(void (^)(NSError *error)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    [[IndyWalletCallbacks sharedInstance] addWalletName:name forRegisteredWalletType:type];
    
    ret = indy_create_wallet(handle,
                               [poolName UTF8String],
                               [name UTF8String],
                               [type UTF8String],
                               [config UTF8String],
                               [credentials UTF8String],
                               IndyWrapperCommon2PCallback
                              );
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

- (NSError *)openWalletWithName:(NSString *)name
                  runtimeConfig:(NSString *)config
                    credentials:(NSString *)credentials
                     completion:(void (^)(NSError *error, IndyHandle walletHandle )) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = indy_open_wallet( handle,
                              [name UTF8String],
                              [config UTF8String],
                              [credentials UTF8String],
                              IndyWrapperCommon3PHCallback
                             );
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

- (NSError *)closeWalletWithHandle:(IndyHandle)walletHandle
                        completion:(void (^)(NSError *error ))handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = indy_close_wallet( handle,
                            walletHandle,
                            IndyWrapperCommon2PCallback
                            );
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

- (NSError *)deleteWalletWithName:(NSString *)walletName
                      credentials:(NSString *)credentials
                       completion:(void (^)(NSError *error ))handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = indy_delete_wallet( handle,
                             [walletName UTF8String],
                             [credentials UTF8String],
                             IndyWrapperCommon2PCallback
                             );
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

@end
