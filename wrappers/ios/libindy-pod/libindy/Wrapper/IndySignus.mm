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

+ (NSError *)replaceKeysWithWalletHandle:(IndyHandle)walletHandle
                                     did:(NSString *)did
                            identityJSON:(NSString *)json
                              completion:(void (^)(NSError *error, NSString *verkey, NSString *pk)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
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
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
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
                          message:(NSData*)message
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

+ (NSError *)verifySignatureWithWalletHandle:(IndyHandle)walletHandle
                                  poolHandle:(IndyHandle)poolHandle
                                         did:(NSString *)did
                                     message:(NSData *)message
                                   signature:(NSData *)signature
                                  completion:(void (^)(NSError *error, BOOL valid)) handler
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

+ (NSError *)encryptWithWalletHandle:(IndyHandle)walletHandle
                                pool:(IndyHandle)poolHandle
                               myDid:(NSString *)myDid
                                 did:(NSString *)did
                             message:(NSData *)message
                          completion:(void (^)(NSError *error, NSData *encryptedMsg, NSData *nonce)) handler
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

+ (NSError *)decryptWithWalletHandle:(IndyHandle)walletHandle
                               myDid:(NSString *)myDid
                                 did:(NSString *)did
                    encryptedMessage:(NSData *)encryptedMessage
                               nonce:(NSData *)nonce
                          completion:(void (^)(NSError *error, NSData *decryptedMessage)) handler
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
