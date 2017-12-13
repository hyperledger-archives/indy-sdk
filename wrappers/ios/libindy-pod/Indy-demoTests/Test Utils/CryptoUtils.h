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

- (NSError *)cryptoBox:(NSData *)message
                 myKey:(NSString *)myKey
              theirKey:(NSString *)theirKey
          walletHandle:(IndyHandle)walletHandle
          outEncrypted:(NSData **)outEncrypted
              outNonce:(NSData **)outNonce;

- (NSError *)cryptoBoxOpen:(IndyHandle)walletHandle
                     myKey:(NSString *)myKey
                  theirKey:(NSString *)theirKey
          encryptedMessage:(NSData *)encryptedMessage
                     nonce:(NSData *)nonce
       outDecryptedMessage:(NSData **)decryptedMessage;

- (NSError *)cryptoBoxSeal:(NSData *)message
                  theirKey:(NSString *)theirKey
              outEncrypted:(NSData **)outEncrypted;

- (NSError *)cryptoBoxSealOpen:(NSData *)encryptedMessage
                         myKey:(NSString *)myKey
                  walletHandle:(IndyHandle)walletHandle
           outDecryptedMessage:(NSData **)decryptedMessage;

@end