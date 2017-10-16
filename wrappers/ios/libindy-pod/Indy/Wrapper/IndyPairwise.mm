//
//  IndyPairwise.m
//  Indy
//

#import "IndyCallbacks.h"
#import "NSError+IndyError.h"
#import "IndyPairwise.h"

@implementation IndyPairwise


+ (NSError *)isPairwiseExistsForDid:(NSString *)theirDid
                       walletHandle:(IndyHandle)walletHandle
                         completion:(void (^)(NSError *error, BOOL exists ))handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_is_pairwise_exists(handle,
                                  walletHandle,
                                  [theirDid UTF8String],
                                  IndyWrapperCommon3PBCallback);
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)createPairwiseForTheirDid:(NSString *)theirDid
                                 myDid:(NSString *)myDid
                              metadata:(NSString *)metadata
                          walletHandle:(IndyHandle)walletHandle
                            completion:(void (^)(NSError *error))completion
{
    indy_error_t ret;
    
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    
    ret = indy_create_pairwise(handle,
                               walletHandle,
                               [theirDid UTF8String],
                               [myDid UTF8String],
                               [metadata UTF8String],
                               IndyWrapperCommon2PCallback);
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}


+ (NSError *)listPairwiseFromWalletHandle:(IndyHandle)walletHandle
                               completion:(void (^)(NSError *error, NSString * listPairwise))completion
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    
    ret = indy_list_pairwise(handle,
                             walletHandle,
                             IndyWrapperCommon3PSCallback);
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)getPairwiseForTheirDid:(NSString *)theirDid
                       walletHandle:(IndyHandle)walletHandle
                         completion:(void (^)(NSError *error, NSString * pairwiseInfoJson))completion
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    
    ret = indy_get_pairwise(handle,
                            walletHandle,
                            [theirDid UTF8String],
                            IndyWrapperCommon3PSCallback);

    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)setPairwiseMetadata:(NSString *)metadata
                     forTheirDid:(NSString *)theirDid
                    walletHandle:(IndyHandle)walletHandle
                      completion:(void (^)(NSError *error))completion
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    
    ret = indy_set_pairwise_metadata(handle,
                                     walletHandle,
                                     [theirDid UTF8String],
                                     [metadata UTF8String],
                                     IndyWrapperCommon2PCallback);
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}
@end
