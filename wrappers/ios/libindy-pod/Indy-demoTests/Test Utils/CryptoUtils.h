//
// Created by DSR on 07/11/2017.
// Copyright (c) 2017 Hyperledger. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTestCase.h>
#import <Indy/IndyTypes.h>


@interface CryptoUtils : XCTestCase

+ (CryptoUtils *)sharedInstance;

- (NSError *)createKeyWithWalletHandle:(IndyHandle)walletHandle
                               keyJson:(NSString *)keyJson
                             outVerkey:(NSString **)outVerkey;

- (NSError *)setMetadata:(NSString *)metadata
                  forKey:(NSString *)key
            walletHandle:(IndyHandle)walletHandle;

- (NSError *)getMetadataForKey:(NSString *)key
                  walletHandle:(IndyHandle)walletHandle
                   outMetadata:(NSString **)outMetadata;

- (NSError *)signMessage:(NSData *)message
                     key:(NSString *)key
            walletHandle:(IndyHandle)walletHandle
            outSignature:(NSData **)outSignature;

- (NSError *)verifySignature:(NSData *)signature
                  forMessage:(NSData *)message
                         key:(NSString *)key
                  outIsValid:(BOOL *)outIsValid;

- (NSError *)authCrypt:(NSData *)message
                 myKey:(NSString *)myKey
              theirKey:(NSString *)theirKey
          walletHandle:(IndyHandle)walletHandle
          outEncrypted:(NSData **)outEncrypted;

- (NSError *)authDecrypt:(NSData *)encryptedMessage
                   myKey:(NSString *)myKey
            walletHandle:(IndyHandle)walletHandle
             outTheirKey:(NSString **)outTheirKey
     outDecryptedMessage:(NSData **)outDecryptedMessage;

- (NSError *)anonCrypt:(NSData *)message
              theirKey:(NSString *)theirKey
          outEncrypted:(NSData **)outEncrypted;

- (NSError *)anonDecrypt:(NSData *)encryptedMessage
                   myKey:(NSString *)myKey
            walletHandle:(IndyHandle)walletHandle
     outDecryptedMessage:(NSData **)outDecryptedMessage;


- (NSError *)packMessage:(NSData *)message
               receivers:(NSString *)receivers
                  sender:(NSString *)sender
            walletHandle:(IndyHandle)walletHandle
                     jwe:(NSData **)jwe;

- (NSError *)unpackMessage:(NSData *)jwe
              walletHandle:(IndyHandle)walletHandle
           unpackedMessage:(NSData **)unpackedMessage;

@end