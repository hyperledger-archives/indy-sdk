//
//  SignusUtils.m
//  Indy-demo
//
//  Created by Anastasia Tarasova on 02.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import "SignusUtils.h"
#import <Indy/Indy.h>
#import "TestUtils.h"
#import "WalletUtils.h"

@implementation SignusUtils

+ (SignusUtils *)sharedInstance
{
    static SignusUtils *instance = nil;
    static dispatch_once_t dispatch_once_block;
    
    dispatch_once(&dispatch_once_block, ^{
        instance = [SignusUtils new];
    });
    
    return instance;
}

+ (NSString *)pool
{
    return @"pool_1";
}

+ (NSData *)message
{
    NSString *messageJson =  @"{\"reqId\":1496822211362017764}";
    return [messageJson dataUsingEncoding:NSUTF8StringEncoding];
}

+ (NSData *)encryptedMessage
{
    const unsigned char bytes[] = {187, 227, 10, 29, 46, 178, 12, 179, 197, 69, 171, 70, 228, 204, 52, 22, 199, 54, 62, 13, 115, 5, 216, 66, 20, 131, 121, 29, 251, 224, 253, 201, 75, 73, 225, 237, 219, 133, 35, 217, 131, 135, 232, 129, 32};
    return [NSData dataWithBytes:bytes length:sizeof(bytes)];
}

+ (NSData *)nonce
{
    const unsigned char bytes[] = {242, 246, 53, 153, 106, 37, 185, 65, 212, 14, 109, 131, 200, 169, 94, 110, 51, 47, 101, 89, 0, 171, 105, 183};
    return [NSData dataWithBytes:bytes length:sizeof(bytes)];
}

+ (NSData *)signature
{
    const unsigned char bytes[] = {169, 215, 8, 225, 7, 107, 110, 9, 193, 162, 202, 214, 162, 66, 238, 211, 63, 209, 12, 196, 8, 211, 55, 27, 120, 94, 204, 147, 53, 104, 103, 61, 60, 249, 237, 127, 103, 46, 220, 223, 10, 95, 75, 53, 245, 210, 241, 151, 191, 41, 48, 30, 9, 16, 78, 252, 157, 206, 210, 145, 125, 133, 109, 11};
    return [NSData dataWithBytes:bytes length:sizeof(bytes)];
}

+ (NSString *)trusteeSeed
{
    return @"000000000000000000000000Trustee1";
}

+ (NSString *)mySeed
{
    return @"00000000000000000000000000000My1";
}

- (NSError *)signWithWalletHandle:(IndyHandle)walletHandle
                         theirDid:(NSString *)theirDid
                          message:(NSData *)message
                     outSignature:(NSData **)signature
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    NSError *ret;


    ret = [IndySignus signMessage:message
                              did:theirDid
                     walletHandle:walletHandle
                       completion:^(NSError *error, NSData *blockSignature)
           {
               err = error;
               if (signature) { *signature = blockSignature; }
               [completionExpectation fulfill];
           }];
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    return err;
}


- (NSError *)createMyDidWithWalletHandle:(IndyHandle)walletHandle
                               myDidJson:(NSString *)myDidJson
                                outMyDid:(NSString **)myDid
                             outMyVerkey:(NSString **)myVerkey
                                 outMyPk:(NSString **)myPk
{
   
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    __block NSString *did = nil;
    __block NSString *verKey = nil;
    __block NSString *pk = nil;
    NSError *ret;

    ret = [IndySignus createAndStoreMyDid:myDidJson
                             walletHandle:walletHandle
                               completion:^(NSError *error, NSString *blockDid, NSString *blockVerKey, NSString *blockPk)
    {
        err = error;
        did = blockDid;
        verKey = blockVerKey;
        pk = blockPk;
        
        [completionExpectation fulfill];
    }];
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    if (myDid) { *myDid = did; }
    if (myVerkey){ *myVerkey = verKey; }
    if (myPk) { *myPk = pk; }
    
    return err;
}

- (NSError *)createAndStoreMyDidWithWalletHandle:(IndyHandle)walletHandle
                                            seed:(NSString *)seed
                                        outMyDid:(NSString **)myDid
                                     outMyVerkey:(NSString **)myVerkey
                                         outMyPk:(NSString **)myPk
{
    
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    __block NSString *did = nil;
    __block NSString *verKey = nil;
    __block NSString *pk = nil;
    NSError *ret;
    
    NSString *myDidJson = (seed) ? [NSString stringWithFormat:@"{\"seed\":\"%@\"}", seed] : @"{}";
    
    ret = [IndySignus createAndStoreMyDid:myDidJson
                             walletHandle:walletHandle
                               completion:^(NSError *error, NSString *blockDid, NSString *blockVerKey, NSString *blockPk)
           {
               err = error;
               did = blockDid;
               verKey = blockVerKey;
               pk = blockPk;
               
               [completionExpectation fulfill];
           }];
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    if (myDid) { *myDid = did; }
    if (myVerkey){ *myVerkey = verKey; }
    if (myPk) { *myPk = pk; }
    
    return err;
}


- (NSError *)storeTheirDidWithWalletHandle: (IndyHandle) walletHandle
                              identityJson: (NSString *)identityJson
{
    
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    NSError *ret;
    
    ret = [IndySignus storeTheirDid:identityJson
                       walletHandle:walletHandle
                         completion:^(NSError *error)
           {
               err = error;
               [completionExpectation fulfill];
           }];
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils longTimeout]];
    
    return err;
}

- (NSError *)storeTheirDidFromPartsWithWalletHandle:(IndyHandle)walletHandle
                                           theirDid:(NSString *)theirDid
                                            theirPk:(NSString *)theirPk
                                        theirVerkey:(NSString *)theirVerkey
                                           endpoint:(NSString *)endpoint
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    
    NSString *theirIdentityJson = [NSString stringWithFormat:@"{"
                                   "\"did\":\"%@\","
                                   "\"verkey\":\"%@\","
                                   "\"endpoint\":\"\%@\"}", theirDid, theirVerkey, endpoint];
    
    NSError *ret = [IndySignus storeTheirDid:theirIdentityJson
                                walletHandle:walletHandle
                                  completion:^(NSError *error)
    {
        err = error;
        [completionExpectation fulfill];
    }];
    
    if (ret.code != Success)
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils longTimeout]];
    
    return err;
}

- (NSError *)replaceKeysStartForDid:(NSString *)did
                       identityJson:(NSString *)identityJson
                       walletHandle:(IndyHandle)walletHandle
                        outMyVerKey:(NSString **)myVerKey
                            outMyPk:(NSString **)myPk
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    __block NSString *verkey;
    __block NSString *pk;
    NSError *ret;
    
    ret = [IndySignus replaceKeysStartForDid:did
                                identityJson:identityJson
                                walletHandle:walletHandle
                                  completion: ^(NSError *error, NSString *blockVerkey, NSString *blockPk)
           {
               err = error;
               verkey = blockVerkey;
               pk = blockPk;
               [completionExpectation fulfill];
           }];
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils longTimeout]];
    
    if (myVerKey) { *myVerKey = verkey; }
    if (myPk) { *myPk = pk; }
    
    return err;
}

- (NSError *)replaceKeysApplyForDid:(NSString *)did
                       walletHandle:(IndyHandle)walletHandle
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    NSError *ret;
    
    ret = [IndySignus replaceKeysApplyForDid:did
                                walletHandle:walletHandle
                                  completion:^(NSError *error)
           {
               err = error;
               [completionExpectation fulfill];
           }];

    if( ret.code != Success)
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils longTimeout]];
    
    return err;
 }
    
- (NSError *)replaceKeysForDid:(NSString *)did
                  identityJson:(NSString *)identityJson
                  walletHandle:(IndyHandle)walletHandle
                    poolHandle:(IndyHandle)poolHandle
                   outMyVerKey:(NSString **)myVerKey
                       outMyPk:(NSString **)myPk
{
    NSError *ret;
    
    NSString *verkey;
    NSString *pk;
    
    ret = [self replaceKeysStartForDid:did
                          identityJson:identityJson
                          walletHandle:walletHandle
                           outMyVerKey:&verkey
                               outMyPk:&pk];
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    NSString *nymRequest;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:did
                                                              targetDid:did
                                                                 verkey:verkey
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    if (ret.code != Success)
    {
        return ret;
    }
    
    NSString *nymResponce;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:did
                                                               requestJson:nymRequest
                                                           outResponseJson:&nymResponce];
    
    if (ret.code != Success)
    {
        return ret;
    }
    
    ret = [self replaceKeysApplyForDid:did
                          walletHandle:walletHandle];
    
    return ret;
}

- (NSError *)verifyWithWalletHandle:(IndyHandle)walletHandle
                         poolHandle:(IndyHandle)poolHandle
                                did:(NSString *)did
                            message:(NSData *)message
                          signature:(NSData *)signature
                        outVerified:(BOOL *)verified
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    NSError *ret;
    
    ret = [IndySignus verifySignature:signature
                           forMessage:message
                                  did:did
                         walletHandle:walletHandle
                           poolHandle:poolHandle
                           completion:^(NSError *error, BOOL valid)
           {
               err = error;
               if (verified) { *verified = valid; }
               [completionExpectation fulfill];
           }];
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils longTimeout]];
    
    return err;
}

- (NSError *)encryptWithWalletHandle:(IndyHandle)walletHandle
                          poolHandle:(IndyHandle)poolHandle
                               myDid:(NSString *)myDid
                                 did:(NSString *)did
                             message:(NSData *)message
                 outEncryptedMessage:(NSData **)encryptedMessage
                            outNonce:(NSData **)nonce
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    NSError *ret;
    
    ret = [IndySignus encryptMessage:message
                               myDid:myDid
                                 did:did
                        walletHandle:walletHandle
                                pool:poolHandle
                          completion:^(NSError *error, NSData *encryptedMsg, NSData *closureNonce)
           {
               err = error;
               if (encryptedMessage) { *encryptedMessage = encryptedMsg; }
               if (nonce) { *nonce = closureNonce; }
               [completionExpectation fulfill];
           }];
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils longTimeout]];
    
    return err;
}

- (NSError *)decryptWithWalletHandle:(IndyHandle)walletHandle
                               myDid:(NSString *)myDid
                                 did:(NSString *)did
                    encryptedMessage:(NSData *)encryptedMessage
                               nonce:(NSData *)nonce
                 outDecryptedMessage:(NSData **)decryptedMessage
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    NSError *ret;
    
    ret = [IndySignus decryptMessage:encryptedMessage
                               myDid:myDid
                                 did:did
                               nonce:nonce
                        walletHandle:walletHandle
                          completion:^(NSError *error, NSData *decryptedMsg)
           {
               err = error;
               if (decryptedMessage) { *decryptedMessage = decryptedMsg; }
               [completionExpectation fulfill];
           }];

    if( ret.code != Success)
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils longTimeout]];
    
    return err;
}


@end
