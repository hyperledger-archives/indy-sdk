//
//  SovrinWallet.m
//  libsovrin
//


#import "SovrinWallet.h"
#import "SovrinCallbacks.h"
#import "sovrin_core.h"
#import "NSError+SovrinError.h"

@implementation SovrinWallet

+ (SovrinWallet*) sharedInstance
{
    static SovrinWallet *instance = nil;
    static dispatch_once_t dispatch_once_block;
    
    dispatch_once(&dispatch_once_block, ^
                  {
                      instance = [SovrinWallet new];
                  });
    
    return instance;
}

- (NSError *)createWalletWithPoolName:(NSString *)poolName
                                 name:(NSString *)name
                                xType:(NSString *)type
                               config:(NSString *)config
                          credentials:(NSString *)credentials
                           completion:(void (^)(NSError *error)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_create_wallet(handle,
                               [poolName UTF8String],
                               [name UTF8String],
                               [type UTF8String],
                               [config UTF8String],
                               [credentials UTF8String],
                               SovrinWrapperCommon2PCallback
                              );
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

- (NSError *)openWalletWithName:(NSString *)name
                  runtimeConfig:(NSString *)config
                    credentials:(NSString *)credentials
                     completion:(void (^)(NSError *error, SovrinHandle walletHandle )) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_open_wallet( handle,
                              [name UTF8String],
                              [config UTF8String],
                              [credentials UTF8String],
                              SovrinWrapperCommon3PHCallback
                             );
    
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

- (NSError *)closeWalletWithHandle:(SovrinHandle)walletHandle
                        completion:(void (^)(NSError *error ))handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_close_wallet( handle,
                               walletHandle,
                               SovrinWrapperCommon2PCallback
                             );
    
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

- (NSError *)deleteWalletWithName:(NSString *)walletName
                      credentials:(NSString *)credentials
                       completion:(void (^)(NSError *error ))handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_delete_wallet( handle,
                                [walletName UTF8String],
                                [credentials UTF8String],
                                SovrinWrapperCommon2PCallback
                               );
    
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

- (NSError *)walletSetSeqNo:(NSNumber *)seqNo
                   forValue:(NSString *)value
               walletHandle:(SovrinHandle)walletHandle
                 completion:(void (^)(NSError *error ))handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_wallet_set_seq_no_for_value( handle,
                                              walletHandle,
                                              [value UTF8String],
                                              [seqNo intValue],
                                              SovrinWrapperCommon2PCallback
                                            );
    
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

@end
