//
// Created by DSR on 03/11/2017.
// Copyright (c) 2017 Hyperledger. All rights reserved.
//

#import <libindy/indy_types.h>
#import "IndyCrypto.h"
#import "indy_mod.h"
#import "IndyCallbacks.h"

@implementation IndyCrypto

+ (void)createKey:(NSString *)keyJson
     walletHandle:(IndyHandle)walletHandle
       completion:(void (^)(NSError *error, NSString *verkey))completion
{
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    indy_error_t ret = indy_create_key(handle, walletHandle, [keyJson UTF8String], IndyWrapperCommonStringCallback);

    [[IndyCallbacks sharedInstance] completeStr:completion forHandle:handle ifError:ret];
}

+ (void)setMetadata:(NSString *)metadata
             forKey:(NSString *)verkey
       walletHandle:(IndyHandle)walletHandle
         completion:(void (^)(NSError *error))completion
{
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    indy_error_t ret = indy_set_key_metadata(handle,
            walletHandle,
            [verkey UTF8String],
            [metadata UTF8String],
            IndyWrapperCommonCallback);

    [[IndyCallbacks sharedInstance] complete:completion forHandle:handle ifError:ret];
}

+ (void)getMetadataForKey:(NSString *)key
             walletHandle:(IndyHandle)walletHandle
               completion:(void (^)(NSError *error, NSString *metadata))completion
{
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    indy_error_t ret = indy_get_key_metadata(handle, walletHandle, [key UTF8String], IndyWrapperCommonStringCallback);

    [[IndyCallbacks sharedInstance] completeStr:completion forHandle:handle ifError:ret];
}

+ (void)signMessage:(NSData *)message
                key:(NSString *)key
       walletHandle:(IndyHandle)walletHandle
         completion:(void (^)(NSError *error, NSData *signature))completion
{
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    uint32_t messageLen = (uint32_t) [message length];
    uint8_t *messageRaw = (uint8_t *) [message bytes];
    indy_error_t ret = indy_crypto_sign(handle,
            walletHandle,
            [key UTF8String],
            messageRaw,
            messageLen,
            IndyWrapperCommonDataCallback);

    [[IndyCallbacks sharedInstance] completeData:completion forHandle:handle ifError:ret];
}

+ (void)verifySignature:(NSData *)signature
             forMessage:(NSData *)message
                    key:(NSString *)key
             completion:(void (^)(NSError *error, BOOL valid))completion
{
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    uint32_t messageLen = (uint32_t) [message length];
    uint8_t *messageRaw = (uint8_t *) [message bytes];
    uint32_t signatureLen = (uint32_t) [signature length];
    uint8_t *signatureRaw = (uint8_t *) [signature bytes];

    indy_error_t ret = indy_crypto_verify(handle,
            [key UTF8String],
            messageRaw,
            messageLen,
            signatureRaw,
            signatureLen,
            IndyWrapperCommonBoolCallback);

    [[IndyCallbacks sharedInstance] completeBool:completion forHandle:handle ifError:ret];
}

+ (void)authCrypt:(NSData *)message
            myKey:(NSString *)myKey
         theirKey:(NSString *)theirKey
     walletHandle:(IndyHandle)walletHandle
        completion:(void (^)(NSError *error, NSData *encyptedMsg))completion
{
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    uint32_t messageLen = (uint32_t) [message length];
    uint8_t *messageRaw = (uint8_t *) [message bytes];

    indy_error_t ret = indy_crypto_auth_crypt(handle,
            walletHandle,
            [myKey UTF8String],
            [theirKey UTF8String],
            messageRaw,
            messageLen,
            IndyWrapperCommonDataCallback);

    [[IndyCallbacks sharedInstance] completeData:completion forHandle:handle ifError:ret];
}

+ (void)authDecrypt:(NSData *)encryptedMessage
        myKey:(NSString *)myKey
        walletHandle:(IndyHandle)walletHandle
           completion:(void (^)(NSError *error, NSString *theirKey, NSData *decryptedMessage))completion
{

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    uint32_t messageLen = (uint32_t) [encryptedMessage length];
    uint8_t *messageRaw = (uint8_t *) [encryptedMessage bytes];

    indy_error_t ret = indy_crypto_auth_decrypt(handle,
            walletHandle,
            [myKey UTF8String],
            messageRaw,
            messageLen,
            IndyWrapperCommonStringDataCallback);

    [[IndyCallbacks sharedInstance] completeStringAndData:completion forHandle:handle ifError:ret];
}

+ (void)anonCrypt:(NSData *)message
             theirKey:(NSString *)theirKey
           completion:(void (^)(NSError *error, NSData *encryptedMsg))completion
{
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    uint32_t messageLen = (uint32_t) [message length];
    uint8_t *messageRaw = (uint8_t *) [message bytes];

    indy_error_t ret = indy_crypto_anon_crypt(handle,
            [theirKey UTF8String],
            messageRaw,
            messageLen,
            IndyWrapperCommonDataCallback);

    [[IndyCallbacks sharedInstance] completeData:completion forHandle:handle ifError:ret];
}

+ (void)anonDecrypt:(NSData *)encryptedMessage
                    myKey:(NSString *)myKey
             walletHandle:(IndyHandle)walletHandle
               completion:(void (^)(NSError *error, NSData *decryptedMessage))completion
{

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    uint32_t messageLen = (uint32_t) [encryptedMessage length];
    uint8_t *messageRaw = (uint8_t *) [encryptedMessage bytes];

    indy_error_t ret = indy_crypto_anon_decrypt(handle,
            walletHandle,
            [myKey UTF8String],
            messageRaw,
            messageLen,
            IndyWrapperCommonDataCallback);

    [[IndyCallbacks sharedInstance] completeData:completion forHandle:handle ifError:ret];
}

+ (void)packMessage:(NSData *)message
          receivers:(NSString *)receivers
             sender:(NSString *)sender
       walletHandle:(IndyHandle)walletHandle
         completion:(void (^)(NSError *error, NSData *jwe))completion
{
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    uint32_t messageLen = (uint32_t) [message length];
    uint8_t *messageRaw = (uint8_t *) [message bytes];

    indy_error_t ret = indy_pack_message(handle,
            walletHandle,
            messageRaw,
            messageLen,
            [receivers UTF8String],
            [sender UTF8String],
            IndyWrapperCommonDataCallback);

    [[IndyCallbacks sharedInstance] completeData:completion forHandle:handle ifError:ret];
}

+ (void)unpackMessage:(NSData *)jwe
         walletHandle:(IndyHandle)walletHandle
           completion:(void (^)(NSError *error, NSData *res))completion
{

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    uint32_t jweLen = (uint32_t) [jwe length];
    uint8_t *jweRaw = (uint8_t *) [jwe bytes];

    indy_error_t ret = indy_unpack_message(handle,
            walletHandle,
            jweRaw,
            jweLen,
            IndyWrapperCommonDataCallback);

    [[IndyCallbacks sharedInstance] completeData:completion forHandle:handle ifError:ret];
}

@end