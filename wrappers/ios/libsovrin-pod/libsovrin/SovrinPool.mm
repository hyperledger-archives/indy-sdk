#import <Foundation/Foundation.h>
#import "SovrinCallbacks.h"
#import "SovrinPool.h"
#import "sovrin_core.h"
#import "NSError+SovrinError.h"

@implementation SovrinPool

+ (NSError*) createPoolWithName:(NSString*) name
                      andConfig:(NSString*) config
                     completion:(void (^)(NSError* error)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_create_pool_ledger_config(handle,
                                           [name UTF8String],
                                           [config UTF8String],
                                           SovrinWrapperCommon2PCallback
                                          );
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) openPoolWithName:(NSString*) name
                    andConfig:(NSString*) config
                   completion:(void (^)(NSError* error, SovrinHandle handle)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_open_pool_ledger(handle,
                                  [name UTF8String],
                                  [config UTF8String],
                                  SovrinWrapperCommon3PHCallback
                                 );
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) refreshPoolWithHandle:(SovrinHandle) SovrinHandle
                        completion:(void (^)(NSError* error)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_refresh_pool_ledger(handle,
                                     (sovrin_handle_t) SovrinHandle,
                                     SovrinWrapperCommon2PCallback
                                    );
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) closePoolWithHandle:(SovrinHandle) SovrinHandle
                      completion:(void (^)(NSError* error)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_close_pool_ledger(handle,
                                   (sovrin_handle_t) SovrinHandle,
                                   SovrinWrapperCommon2PCallback
                                  );
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) deletePoolWithName:(NSString*) name
                     completion:(void (^)(NSError* error)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_delete_pool_ledger_config(handle,
                                           [name UTF8String],
                                           SovrinWrapperCommon2PCallback
                                          );
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

@end

