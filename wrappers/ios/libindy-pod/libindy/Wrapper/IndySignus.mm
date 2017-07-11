//
//  IndySignus.m
//  libindy
//

#import "IndySignus.h"
#import "IndyCallbacks.h"
#import "indy_core.h"
#import "NSError+IndyError.h"

@implementation IndySignus

+ (NSError *)createAndStoreMyDidWithWalletHandle:(IndyHandle)walletHandle
                                         didJSON:(NSString *)didJson
                                      completion:(void (^)(NSError *error, NSString *did, NSString *verkey, NSString *pk)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = indy_create_and_store_my_did( handle,
                                          walletHandle,
                                          [didJson UTF8String],
                                          IndyWrapperCommon5PCallback );
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
    
}

+ (NSError *)replaceKeysWithWalletHandle:(IndyHandle)walletHandle
                                     did:(NSString *)did
                            identityJSON:(NSString *)json
                              completion:(void (^)(NSError *error, NSString *verkey, NSString *pk)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = indy_replace_keys( handle,
                               walletHandle,
                               [did UTF8String],
                               [json UTF8String],
                               IndyWrapperCommon4PCallback
                              );
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)storeTheirDidWithWalletHandle:(IndyHandle)walletHandle
                              identityJSON:(NSString *)json
                                completion:(void (^)(NSError *error)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = indy_store_their_did( handle,
                                  walletHandle,
                                  [json UTF8String],
                                  IndyWrapperCommon2PCallback
                                );
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)signWithWalletHandle:(IndyHandle)walletHandle
                              did:(NSString *)did
                              msg:(NSString *)msg
                       completion:(void (^)(NSError *error, NSString *signature)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = indy_sign( handle,
                       walletHandle,
                       [did UTF8String],
                       [msg UTF8String],
                       IndyWrapperCommon3PSCallback
                     );
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)verifySignatureWithWalletHandle:(IndyHandle)walletHandle
                                  poolHandle:(IndyHandle)poolHandle
                                         did:(NSString *)did
                                   signature:(NSString *)signature
                                  completion:(void (^)(NSError *error, BOOL valid)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = indy_verify_signature(handle,
                                  walletHandle,
                                  poolHandle,
                                  [did UTF8String],
                                  [signature UTF8String],
                                  IndyWrapperCommon3PBCallback);
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)encryptWithWalletHandle:(IndyHandle)walletHandle
                                pool:(IndyHandle)poolHandle
                               myDid:(NSString *)myDid
                                 did:(NSString *)did
                                 msg:(NSString *)msg
                          completion:(void (^)(NSError *error, NSString *encryptedMsg, NSString *nonce)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = indy_encrypt( handle,
                          walletHandle,
                          poolHandle,
                          [myDid UTF8String],
                          [did UTF8String],
                          [msg UTF8String],
                          IndyWrapperCommon4PCallback
                        );
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)decryptWithWalletHandle:(IndyHandle)walletHandle
                               myDid:(NSString *)myDid
                                 did:(NSString *)did
                        encryptedMsg:(NSString *)msg
                               nonce:(NSString *)nonce
                          completion:(void (^)(NSError *error, NSString *decryptedMsg)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = indy_decrypt( handle,
                          walletHandle,
                          [myDid UTF8String],
                          [did UTF8String],
                          [msg UTF8String],
                          [nonce UTF8String],
                          IndyWrapperCommon3PSCallback
                        );
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

@end
