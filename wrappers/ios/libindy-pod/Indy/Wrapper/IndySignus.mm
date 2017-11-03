//
//  IndySignus.m
//  libindy
//

#import "IndySignus.h"
#import "IndyCallbacks.h"

@implementation IndySignus

+ (void)createAndStoreMyDid:(NSString *)didJson
               walletHandle:(IndyHandle)walletHandle
                 completion:(void (^)(NSError *error,
                                      NSString *did,
                                      NSString *verkey)) completion
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    
    ret = indy_create_and_store_my_did(handle,
                                       walletHandle,
                                       [didJson UTF8String],
                                       IndyWrapperCommon4PCallback);
    [[IndyCallbacks sharedInstance] complete2Str:completion forHandle:handle ifError:ret];
}

+ (void)replaceKeysStartForDid:(NSString *)did
                  identityJson:(NSString *)identityJson
                  walletHandle:(IndyHandle)walletHandle
                    completion:(void (^)(NSError *error,
                                         NSString *verkey)) completion
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    
    ret = indy_replace_keys_start(handle,
                                  walletHandle,
                                  [did UTF8String],
                                  [identityJson UTF8String],
                                  IndyWrapperCommon3PSCallback);
    [[IndyCallbacks sharedInstance] completeStr:completion forHandle:handle ifError:ret];
}

+ (void)replaceKeysApplyForDid:(NSString *)did
                  walletHandle:(IndyHandle)walletHandle
                    completion:(void (^)(NSError *error)) completion
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    
    ret = indy_replace_keys_apply(handle,
                                  walletHandle,
                                  [did UTF8String],
                                  IndyWrapperCommon2PCallback);
    [[IndyCallbacks sharedInstance] complete:completion forHandle:handle ifError:ret];
}

+ (void)storeTheirDid:(NSString *)identityJSON
         walletHandle:(IndyHandle)walletHandle
           completion:(void (^)(NSError *error)) completion
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    
    ret = indy_store_their_did( handle,
                               walletHandle,
                               [identityJSON UTF8String],
                               IndyWrapperCommon2PCallback
                               );
    [[IndyCallbacks sharedInstance] complete:completion forHandle:handle ifError:ret];
}

+ (void)keyForDid:(NSString *)did
       poolHandle:(IndyHandle)poolHandle
     walletHandle:(IndyHandle)walletHandle
       completion:(void (^)(NSError *error, NSString *key))completion
{
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    indy_error_t ret = indy_key_for_did(handle, poolHandle, walletHandle, [did UTF8String], IndyWrapperCommon3PSCallback);

    [[IndyCallbacks sharedInstance] completeStr:completion forHandle:handle ifError:ret];
}

+ (void)setEndpointAddress:(NSString *)address
              transportKey:(NSString *)transportKey
                    forDid:(NSString *)did
              walletHandle:(IndyHandle)walletHandle
                completion:(void (^)(NSError *error))completion
{
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    indy_error_t ret = indy_set_endpoint_for_did(handle,
            walletHandle,
            [did UTF8String],
            [address UTF8String],
            [transportKey UTF8String],
            IndyWrapperCommon2PCallback);

    [[IndyCallbacks sharedInstance] complete:completion forHandle:handle ifError:ret];
}

+ (void)getEndpointForDid:(NSString *)did
             walletHandle:(IndyHandle)walletHandle
               completion:(void (^)(NSError *error, NSString *address, NSString *transportKey))completion
{
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    indy_error_t ret = indy_get_key_metadata(handle, walletHandle, [did UTF8String], IndyWrapperCommon3PSCallback);

    [[IndyCallbacks sharedInstance] complete2Str:completion forHandle:handle ifError:ret];
}

+ (void)setMetadata:(NSString *)metadata
             forDid:(NSString *)did
       walletHandle:(IndyHandle)walletHandle
         completion:(void (^)(NSError *error))completion
{
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    indy_error_t ret = indy_set_key_metadata(handle,
            walletHandle,
            [did UTF8String],
            [metadata UTF8String],
            IndyWrapperCommon2PCallback);

    [[IndyCallbacks sharedInstance] complete:completion forHandle:handle ifError:ret];
}

+ (void)getMetadataForDid:(NSString *)did
             walletHandle:(IndyHandle)walletHandle
               completion:(void (^)(NSError *error, NSString *metadata))completion
{
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    indy_error_t ret = indy_get_key_metadata(handle, walletHandle, [did UTF8String], IndyWrapperCommon3PSCallback);

    [[IndyCallbacks sharedInstance] completeStr:completion forHandle:handle ifError:ret];
}


+ (void)signMessage:(NSData*)message
                did:(NSString *)did
       walletHandle:(IndyHandle)walletHandle
         completion:(void (^)(NSError *error,
                              NSData *signature)) completion
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    
    uint32_t messageLen = (uint32_t)[message length];
    uint8_t *messageRaw = (uint8_t *)[message bytes];
    ret = indy_sign(handle,
                    walletHandle,
                    [did UTF8String],
                    messageRaw,
                    messageLen,
                    IndyWrapperCommon4PDataCallback);
    [[IndyCallbacks sharedInstance] completeData:completion forHandle:handle ifError:ret];
}

+ (void)verifySignature:(NSData *)signature
             forMessage:(NSData *)message
                    did:(NSString *)did
           walletHandle:(IndyHandle)walletHandle
             poolHandle:(IndyHandle)poolHandle
             completion:(void (^)(NSError *error,
                                  BOOL valid)) completion
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    
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
    [[IndyCallbacks sharedInstance] completeBool:completion forHandle:handle ifError:ret];
}

+ (void)encryptMessage:(NSData *)message
                 myDid:(NSString *)myDid
                   did:(NSString *)did
          walletHandle:(IndyHandle)walletHandle
                  pool:(IndyHandle)poolHandle
            completion:(void (^)(NSError *error,
                                 NSData *encryptedMsg,
                                 NSData *nonce)) completion
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    
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
    [[IndyCallbacks sharedInstance] complete2Data:completion forHandle:handle ifError:ret];
}

+ (void)decryptMessage:(NSData *)encryptedMessage
                 myDid:(NSString *)myDid
                   did:(NSString *)did
                 nonce:(NSData *)nonce
          walletHandle:(IndyHandle)walletHandle
            poolHandle:(IndyHandle)poolHandle
            completion:(void (^)(NSError *error,
                                 NSData *decryptedMessage)) completion
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    
    uint32_t messageLen = (uint32_t)[encryptedMessage length];
    uint8_t *messageRaw = (uint8_t *)[encryptedMessage bytes];
    uint32_t nonceLen = (uint32_t)[nonce length];
    uint8_t *nonceRaw = (uint8_t *)[nonce bytes];
    
    ret = indy_decrypt(handle,
                       walletHandle,
                       poolHandle,
                       [myDid UTF8String],
                       [did UTF8String],
                        messageRaw,
                        messageLen,
                        nonceRaw,
                        nonceLen,
                        IndyWrapperCommon4PDataCallback);
    [[IndyCallbacks sharedInstance] completeData:completion forHandle:handle ifError:ret];
}

+ (void)encryptMessageSealed:(NSData *)message
                         did:(NSString *)did
                walletHandle:(IndyHandle)walletHandle
                        pool:(IndyHandle)poolHandle
                  completion:(void (^)(NSError *error, NSData *encryptedMsg))completion
{
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    uint32_t messageLen = (uint32_t) [message length];
    uint8_t *messageRaw = (uint8_t *) [message bytes];

    ret = indy_encrypt_sealed(handle,
            walletHandle,
            [did UTF8String],
            messageRaw,
            messageLen,
            IndyWrapperCommon4PDataCallback);
    [[IndyCallbacks sharedInstance] completeData:completion forHandle:handle ifError:ret];
}

+ (void)decryptMessageSealed:(NSData *)encryptedMessage
                         did:(NSString *)did
                walletHandle:(IndyHandle)walletHandle
                  completion:(void (^)(NSError *error, NSData *decryptedMessage))completion
{
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    uint32_t messageLen = (uint32_t) [encryptedMessage length];
    uint8_t *messageRaw = (uint8_t *) [encryptedMessage bytes];

    ret = indy_decrypt_sealed(handle,
            walletHandle,
            [did UTF8String],
            messageRaw,
            messageLen,
            IndyWrapperCommon4PDataCallback);
    [[IndyCallbacks sharedInstance] completeData:completion forHandle:handle ifError:ret];
}

@end
