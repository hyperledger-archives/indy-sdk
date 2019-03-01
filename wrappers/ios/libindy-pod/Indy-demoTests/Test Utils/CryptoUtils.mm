//
// Created by DSR on 07/11/2017.
// Copyright (c) 2017 Hyperledger. All rights reserved.
//

#import "CryptoUtils.h"
#import <Indy/IndyCrypto.h>
#import "TestUtils.h"


@implementation CryptoUtils

+ (CryptoUtils *)sharedInstance
{
    static CryptoUtils *instance = nil;
    static dispatch_once_t dispatch_once_block;

    dispatch_once(&dispatch_once_block, ^{
        instance = [CryptoUtils new];
    });

    return instance;
}

- (NSError *)createKeyWithWalletHandle:(IndyHandle)walletHandle
                               keyJson:(NSString *)keyJson
                             outVerkey:(NSString **)outVerkey
{
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyCrypto createKey:keyJson walletHandle:walletHandle completion:^(NSError *error, NSString *verkey) {
        err = error;
        if (outVerkey) *outVerkey = verkey;
        [completionExpectation fulfill];
    }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    return err;
}

- (NSError *)setMetadata:(NSString *)metadata
                  forKey:(NSString *)key
            walletHandle:(IndyHandle)walletHandle
{
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyCrypto setMetadata:metadata forKey:key walletHandle:walletHandle completion:^(NSError *error) {
        err = error;
        [completionExpectation fulfill];
    }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    return err;
}

- (NSError *)getMetadataForKey:(NSString *)key
                  walletHandle:(IndyHandle)walletHandle
                   outMetadata:(NSString **)outMetadata
{
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyCrypto getMetadataForKey:key walletHandle:walletHandle completion:^(NSError *error, NSString *metadata) {
        err = error;
        if (outMetadata) *outMetadata = metadata;
        [completionExpectation fulfill];
    }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    return err;
}

- (NSError *)signMessage:(NSData *)message
                     key:(NSString *)key
            walletHandle:(IndyHandle)walletHandle
            outSignature:(NSData **)outSignature
{
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyCrypto signMessage:message key:key walletHandle:walletHandle completion:^(NSError *error, NSData *signature) {
        err = error;
        if (outSignature) *outSignature = signature;
        [completionExpectation fulfill];
    }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    return err;
}

- (NSError *)verifySignature:(NSData *)signature
                  forMessage:(NSData *)message
                         key:(NSString *)key
                  outIsValid:(BOOL *)outIsValid
{
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyCrypto verifySignature:signature forMessage:message key:key completion:^(NSError *error, BOOL isValid) {
        err = error;
        if (outIsValid) *outIsValid = isValid;
        [completionExpectation fulfill];
    }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    return err;
}

- (NSError *)authCrypt:(NSData *)message
                 myKey:(NSString *)myKey
              theirKey:(NSString *)theirKey
          walletHandle:(IndyHandle)walletHandle
          outEncrypted:(NSData **)outEncrypted
{
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyCrypto authCrypt:message myKey:myKey theirKey:theirKey walletHandle:walletHandle completion:^(NSError *error, NSData *encrypted) {
        err = error;
        if (outEncrypted) *outEncrypted = encrypted;
        [completionExpectation fulfill];
    }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    return err;
}

- (NSError *)authDecrypt:(NSData *)encryptedMessage
        myKey:(NSString *)myKey
            walletHandle:(IndyHandle)walletHandle
             outTheirKey:(NSString **)outTheirKey
     outDecryptedMessage:(NSData **)outDecryptedMessage {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyCrypto authDecrypt:encryptedMessage myKey:myKey walletHandle:walletHandle completion:^(NSError *error, NSString *theirKey, NSData *decryptedMsg) {
        err = error;
        if (outTheirKey) *outTheirKey = theirKey;
        if (outDecryptedMessage) {*outDecryptedMessage = decryptedMsg;}
        [completionExpectation fulfill];
    }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    return err;
}

- (NSError *)anonCrypt:(NSData *)message
                  theirKey:(NSString *)theirKey
              outEncrypted:(NSData **)outEncrypted
{
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyCrypto anonCrypt:message theirKey:theirKey completion:^(NSError *error, NSData *encrypted) {
        err = error;
        if (outEncrypted) *outEncrypted = encrypted;
        [completionExpectation fulfill];
    }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    return err;
}

- (NSError *)anonDecrypt:(NSData *)encryptedMessage
                         myKey:(NSString *)myKey
                  walletHandle:(IndyHandle)walletHandle
           outDecryptedMessage:(NSData **)decryptedMessage
{
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyCrypto anonDecrypt:encryptedMessage myKey:myKey walletHandle:walletHandle completion:^(NSError *error, NSData *decryptedMsg) {
        err = error;
        if (decryptedMessage) {*decryptedMessage = decryptedMsg;}
        [completionExpectation fulfill];
    }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    return err;
}

- (NSError *)packMessage:(NSData *)message
               receivers:(NSString *)receivers
                  sender:(NSString *)sender
            walletHandle:(IndyHandle)walletHandle
                     jwe:(NSData **)jwe
{
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyCrypto packMessage:message receivers:receivers sender:sender walletHandle:walletHandle completion:^(NSError *error, NSData *outJwe) {
        err = error;
        if (jwe) *jwe = outJwe;
        [completionExpectation fulfill];
    }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    return err;
}

- (NSError *)unpackMessage:(NSData *)jwe
              walletHandle:(IndyHandle)walletHandle
           unpackedMessage:(NSData **)unpackedMessage {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyCrypto unpackMessage:jwe walletHandle:walletHandle completion:^(NSError *error, NSData *outRes) {
        err = error;
        if (unpackedMessage) {*unpackedMessage = outRes;}
        [completionExpectation fulfill];
    }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    return err;
}

@end
