//
//  IndySignus.m
//  libindy
//

#import "IndySignus.h"
#import "IndyCallbacks.h"
#import "indy_core.h"
#import "NSError+IndyError.h"

@implementation IndySignus

+ (NSError *)createAndStoreMyDid:(NSString *)didJson
                    walletHandle:(IndyHandle)walletHandle
                                      completion:(void (^)(NSError *error, NSString *did, NSString *verkey, NSString *pk)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
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

+ (NSError *)replaceKeysStartForDid:(NSString *)did
                       identityJson:(NSString *)identityJson
                       walletHandle:(IndyHandle)walletHandle
                         completion:(void (^)(NSError *error,
                                              NSString *verkey,
                                              NSString *pk)) completion
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    
    ret = indy_replace_keys_start(handle,
                                  walletHandle,
                                  [did UTF8String],
                                  [identityJson UTF8String],
                                  IndyWrapperCommon4PCallback);
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)replaceKeysApplyForDid:(NSString *)did
                       walletHandle:(IndyHandle)walletHandle
                         completion:(void (^)(NSError *error)) completion
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    
    ret = indy_replace_keys_apply(handle,
                                  walletHandle,
                                  [did UTF8String],
                                  IndyWrapperCommon2PCallback);
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)storeTheirDid:(NSString *)identityJSON
              walletHandle:(IndyHandle)walletHandle
                completion:(void (^)(NSError *error)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_store_their_did( handle,
                               walletHandle,
                               [identityJSON UTF8String],
                               IndyWrapperCommon2PCallback
                               );
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)signMessage:(NSData*)message
                     did:(NSString *)did
            walletHandle:(IndyHandle)walletHandle
              completion:(void (^)(NSError *error,
                                   NSData *signature)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    uint32_t messageLen = (uint32_t)[message length];
    uint8_t *messageRaw = (uint8_t *)[message bytes];
    ret = indy_sign(handle,
                    walletHandle,
                    [did UTF8String],
                    messageRaw,
                    messageLen,
                    IndyWrapperCommon4PDataCallback);
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)verifySignature:(NSData *)signature
                  forMessage:(NSData *)message
                         did:(NSString *)did
                walletHandle:(IndyHandle)walletHandle
                  poolHandle:(IndyHandle)poolHandle
                  completion:(void (^)(NSError *error,
                                       BOOL valid)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    uint32_t messageLen = (uint32_t)[message length];
    uint8_t *messageRaw = (uint8_t *)[message bytes];
    uint32_t signatureLen = (uint32_t)[signature length];
    uint8_t *signatureRaw = (uint8_t *)[signature bytes];
    
    ret = indy_verify_signature(handle,
                                walletHandle,
                                poolHandle,
                                [did UTF8String],
                                messageRaw,
                                messageLen,
                                signatureRaw,
                                signatureLen,
                                IndyWrapperCommon3PBCallback);
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)encryptMessage:(NSData *)message
                      myDid:(NSString *)myDid
                        did:(NSString *)did
               walletHandle:(IndyHandle)walletHandle
                       pool:(IndyHandle)poolHandle
                 completion:(void (^)(NSError *error,
                                      NSData *encryptedMsg,
                                      NSData *nonce)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    uint32_t messageLen = (uint32_t)[message length];
    uint8_t *messageRaw = (uint8_t *)[message bytes];

    ret = indy_encrypt(handle,
                       walletHandle,
                       poolHandle,
                       [myDid UTF8String],
                       [did UTF8String],
                       messageRaw,
                       messageLen,
                       IndyWrapperCommon6PDataCallback);
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)decryptMessage:(NSData *)encryptedMessage
                      myDid:(NSString *)myDid
                        did:(NSString *)did
                      nonce:(NSData *)nonce
               walletHandle:(IndyHandle)walletHandle
                 completion:(void (^)(NSError *error,
                                      NSData *decryptedMessage)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    uint32_t messageLen = (uint32_t)[encryptedMessage length];
    uint8_t *messageRaw = (uint8_t *)[encryptedMessage bytes];
    uint32_t nonceLen = (uint32_t)[nonce length];
    uint8_t *nonceRaw = (uint8_t *)[nonce bytes];
    
    ret = indy_decrypt(handle,
                       walletHandle,
                       [myDid UTF8String],
                       [did UTF8String],
                        messageRaw,
                        messageLen,
                        nonceRaw,
                        nonceLen,
                        IndyWrapperCommon4PDataCallback);
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

@end
