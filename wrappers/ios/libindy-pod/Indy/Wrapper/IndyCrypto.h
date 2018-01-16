//
// Created by DSR on 03/11/2017.
// Copyright (c) 2017 Hyperledger. All rights reserved.
//

#import <Foundation/Foundation.h>
#import "IndyTypes.h"

@interface IndyCrypto : NSObject

+ (void)createKey:(NSString *)keyJson
     walletHandle:(IndyHandle)walletHandle
       completion:(void (^)(NSError *error, NSString *verkey))completion;

+ (void)setMetadata:(NSString *)metadata
             forKey:(NSString *)verkey
       walletHandle:(IndyHandle)walletHandle
         completion:(void (^)(NSError *error))completion;

+ (void)getMetadataForKey:(NSString *)key
             walletHandle:(IndyHandle)walletHandle
               completion:(void (^)(NSError *error, NSString *metadata))completion;

+ (void)signMessage:(NSData *)message
                key:(NSString *)key
       walletHandle:(IndyHandle)walletHandle
         completion:(void (^)(NSError *error, NSData *signature))completion;

+ (void)verifySignature:(NSData *)signature
             forMessage:(NSData *)message
                    key:(NSString *)key
             completion:(void (^)(NSError *error, BOOL valid))completion;

+ (void)authCrypt:(NSData *)message
            myKey:(NSString *)myKey
         theirKey:(NSString *)theirKey
     walletHandle:(IndyHandle)walletHandle
       completion:(void (^)(NSError *error, NSData *encryptedMsg))completion;

+ (void)authDecrypt:(NSData *)encryptedMessage
        myKey:(NSString *)myKey
        walletHandle:(IndyHandle)walletHandle
        completion:(void (^)(NSError *error, NSString *theirKey, NSData *decryptedMessage))completion;

+ (void)anonCrypt:(NSData *)message
             theirKey:(NSString *)theirKey
           completion:(void (^)(NSError *error, NSData *encryptedMsg))completion;

+ (void)anonDecrypt:(NSData *)encryptedMessage
                    myKey:(NSString *)myKey
             walletHandle:(IndyHandle)walletHandle
               completion:(void (^)(NSError *error, NSData *decryptedMessage))completion;

@end