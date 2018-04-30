//
//  IndyPairwise.m
//  Indy
//

#import "IndyCallbacks.h"
#import "NSError+IndyError.h"
#import "IndyPairwise.h"

@implementation IndyPairwise


+ (void)isPairwiseExistsForDid:(NSString *)theirDid
                  walletHandle:(IndyHandle)walletHandle
                    completion:(void (^)(NSError *error, BOOL exists ))completion
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    
    ret = indy_is_pairwise_exists(handle,
            walletHandle,
            [theirDid UTF8String],
            IndyWrapperCommonBoolCallback);
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
        
        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError: ret], false);
        });
    }
}

+ (void)createPairwiseForTheirDid:(NSString *)theirDid
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
            IndyWrapperCommonCallback);
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
        
        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError: ret]);
        });
    }
}


+ (void)listPairwiseFromWalletHandle:(IndyHandle)walletHandle
                          completion:(void (^)(NSError *error, NSString * listPairwise))completion
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    
    ret = indy_list_pairwise(handle,
            walletHandle,
            IndyWrapperCommonStringCallback);
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
        
        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError: ret], nil);
        });
    }
}

+ (void)getPairwiseForTheirDid:(NSString *)theirDid
                  walletHandle:(IndyHandle)walletHandle
                    completion:(void (^)(NSError *error, NSString *pairwiseInfoJson))completion
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    
    ret = indy_get_pairwise(handle,
            walletHandle,
            [theirDid UTF8String],
            IndyWrapperCommonStringCallback);

    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
        
        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError: ret], nil);
        });
    }
}

+ (void)setPairwiseMetadata:(NSString *)metadata
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
            IndyWrapperCommonCallback);
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
        
        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError: ret]);
        });
    }
}
@end
