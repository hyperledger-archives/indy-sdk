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
    indy_error_t ret = indy_create_key(handle, walletHandle, [keyJson UTF8String], IndyWrapperCommon3PSCallback);

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
            IndyWrapperCommon2PCallback);

    [[IndyCallbacks sharedInstance] complete:completion forHandle:handle ifError:ret];
}

+ (void)getMetadataForKey:(NSString *)key
             walletHandle:(IndyHandle)walletHandle
               completion:(void (^)(NSError *error, NSString *metadata))completion
{
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    indy_error_t ret = indy_get_key_metadata(handle, walletHandle, [key UTF8String], IndyWrapperCommon3PSCallback);

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
            IndyWrapperCommon4PDataCallback);

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
            IndyWrapperCommon3PBCallback);

    [[IndyCallbacks sharedInstance] completeBool:completion forHandle:handle ifError:ret];
}

+ (void)cryptoBox:(NSData *)message
            myKey:(NSString *)myKey
         theirKey:(NSString *)theirKey
     walletHandle:(IndyHandle)walletHandle
       completion:(void (^)(NSError *error, NSData *encryptedMsg, NSData *nonce))completion
{
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    uint32_t messageLen = (uint32_t) [message length];
    uint8_t *messageRaw = (uint8_t *) [message bytes];

    indy_error_t ret = indy_crypto_box(handle,
            walletHandle,
            [myKey UTF8String],
            [theirKey UTF8String],
            messageRaw,
            messageLen,
            IndyWrapperCommon6PDataCallback);

    [[IndyCallbacks sharedInstance] complete2Data:completion forHandle:handle ifError:ret];
}

+ (void)cryptoBoxOpen:(NSData *)encryptedMessage
                myKey:(NSString *)myKey
             theirKey:(NSString *)theirKey
                nonce:(NSData *)nonce
         walletHandle:(IndyHandle)walletHandle
           completion:(void (^)(NSError *error, NSData *decryptedMessage))completion
{

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    uint32_t messageLen = (uint32_t) [encryptedMessage length];
    uint8_t *messageRaw = (uint8_t *) [encryptedMessage bytes];
    uint32_t nonceLen = (uint32_t) [nonce length];
    uint8_t *nonceRaw = (uint8_t *) [nonce bytes];

    indy_error_t ret = indy_crypto_box_open(handle,
            walletHandle,
            [myKey UTF8String],
            [theirKey UTF8String],
            messageRaw,
            messageLen,
            nonceRaw,
            nonceLen,
            IndyWrapperCommon4PDataCallback);

    [[IndyCallbacks sharedInstance] completeData:completion forHandle:handle ifError:ret];
}

+ (void)cryptoBoxSeal:(NSData *)message
             theirKey:(NSString *)theirKey
           completion:(void (^)(NSError *error, NSData *encryptedMsg))completion
{
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    uint32_t messageLen = (uint32_t) [message length];
    uint8_t *messageRaw = (uint8_t *) [message bytes];

    indy_error_t ret = indy_crypto_box_seal(handle,
            [theirKey UTF8String],
            messageRaw,
            messageLen,
            IndyWrapperCommon4PDataCallback);

    [[IndyCallbacks sharedInstance] completeData:completion forHandle:handle ifError:ret];
}

+ (void)cryptoBoxSealOpen:(NSData *)encryptedMessage
                    myKey:(NSString *)myKey
             walletHandle:(IndyHandle)walletHandle
               completion:(void (^)(NSError *error, NSData *decryptedMessage))completion
{

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    uint32_t messageLen = (uint32_t) [encryptedMessage length];
    uint8_t *messageRaw = (uint8_t *) [encryptedMessage bytes];

    indy_error_t ret = indy_crypto_box_seal_open(handle,
            walletHandle,
            [myKey UTF8String],
            messageRaw,
            messageLen,
            IndyWrapperCommon4PDataCallback);

    [[IndyCallbacks sharedInstance] completeData:completion forHandle:handle ifError:ret];
}


@end