//
//  IndySdk.h
//  vcx
//
//  Created by Norman Jarvis on 2/18/19.
//  Copyright © 2019 GuestUser. All rights reserved.
//

#ifndef IndySdk_h
#define IndySdk_h

#import <Foundation/Foundation.h>
//#import "vcx.h"
//#include "vcx.h"
#import "IndyTypes.h"

@interface IndySdk : NSObject

/**
 Encrypts a message by anonymous-encryption scheme.
 
 Sealed boxes are designed to anonymously send messages to a Recipient given its public key.
 Only the Recipient can decrypt these messages, using its private key.
 While the Recipient can verify the integrity of the message, it cannot verify the identity of the Sender.
 
 Note to use DID keys with this function you can call keyForDid to get key id (verkey)
 for specific DID.
 
 @param theirKey verkey of message recipient
 @param message a message to be encrypted
 Returns encrypted message
 */
+ (void)anonCrypt:(NSData *)message
         theirKey:(NSString *)theirKey
       completion:(void (^)(NSError *error, NSData *encryptedMsg))completion;

/**
 Decrypts a message by anonymous-encryption scheme.
 
 Sealed boxes are designed to anonymously send messages to a Recipient given its public key.
 Only the Recipient can decrypt these messages, using its private key.
 While the Recipient can verify the integrity of the message, it cannot verify the identity of the Sender.
 
 Note to use DID keys with this function you can call keyForDid to get key id (verkey)
 for specific DID.
 
 @param walletHandle       The wallet.
 @param myKey  Id (verkey) of my key. The key must be created by calling createKey or createAndStoreMyDid
 @param encryptedMessage encrypted message
 Returns decrypted message as an array of bytes.
 */
+ (void)anonDecrypt:(NSData *)encryptedMessage
              myKey:(NSString *)myKey
       walletHandle:(IndyHandle)walletHandle
         completion:(void (^)(NSError *error, NSData *decryptedMessage))completion;


@end

#endif /* IndySdk_h */
