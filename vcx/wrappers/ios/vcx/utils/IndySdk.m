//
//  IndySdk.m
//  vcx
//
//  Created by Norman Jarvis on 2/18/19.
//  Copyright © 2019 GuestUser. All rights reserved.
//


#include "IndySdk.h"
#include "IndyCallbacks.h"
//#include "vcx.h"
#include "indy_types.h"
#include "indy_crypto.h"

@implementation IndySdk


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


@end

