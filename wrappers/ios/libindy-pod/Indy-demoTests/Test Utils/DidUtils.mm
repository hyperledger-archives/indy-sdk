//
//  DidUtils.m
//  Indy-demo
//
//  Created by Anastasia Tarasova on 02.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import "DidUtils.h"
#import <Indy/Indy.h>
#import "TestUtils.h"
#import "WalletUtils.h"

@implementation DidUtils

+ (DidUtils *)sharedInstance {
    static DidUtils *instance = nil;
    static dispatch_once_t dispatch_once_block;

    dispatch_once(&dispatch_once_block, ^{
        instance = [DidUtils new];
    });

    return instance;
}

+ (NSString *)pool {
    return @"pool_1";
}

+ (NSData *)message {
    NSString *messageJson = @"{\"reqId\":1496822211362017764}";
    return [messageJson dataUsingEncoding:NSUTF8StringEncoding];
}

+ (NSData *)encryptedMessage {
    const unsigned char bytes[] = {187, 227, 10, 29, 46, 178, 12, 179, 197, 69, 171, 70, 228, 204, 52, 22, 199, 54, 62, 13, 115, 5, 216, 66, 20, 131, 121, 29, 251, 224, 253, 201, 75, 73, 225, 237, 219, 133, 35, 217, 131, 135, 232, 129, 32};
    return [NSData dataWithBytes:bytes length:sizeof(bytes)];
}

+ (NSData *)nonce {
    const unsigned char bytes[] = {242, 246, 53, 153, 106, 37, 185, 65, 212, 14, 109, 131, 200, 169, 94, 110, 51, 47, 101, 89, 0, 171, 105, 183};
    return [NSData dataWithBytes:bytes length:sizeof(bytes)];
}

+ (NSData *)signature {
    const unsigned char bytes[] = {169, 215, 8, 225, 7, 107, 110, 9, 193, 162, 202, 214, 162, 66, 238, 211, 63, 209, 12, 196, 8, 211, 55, 27, 120, 94, 204, 147, 53, 104, 103, 61, 60, 249, 237, 127, 103, 46, 220, 223, 10, 95, 75, 53, 245, 210, 241, 151, 191, 41, 48, 30, 9, 16, 78, 252, 157, 206, 210, 145, 125, 133, 109, 11};
    return [NSData dataWithBytes:bytes length:sizeof(bytes)];
}

- (NSError *)createMyDidWithWalletHandle:(IndyHandle)walletHandle
                               myDidJson:(NSString *)myDidJson
                                outMyDid:(NSString **)myDid
                             outMyVerkey:(NSString **)myVerkey {

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *did = nil;
    __block NSString *verKey = nil;

    [IndyDid createAndStoreMyDid:myDidJson
                    walletHandle:walletHandle
                      completion:^(NSError *error, NSString *blockDid, NSString *blockVerKey) {
                          err = error;
                          did = blockDid;
                          verKey = blockVerKey;

                          [completionExpectation fulfill];
                      }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    if (myDid) {*myDid = did;}
    if (myVerkey) {*myVerkey = verKey;}

    return err;
}

- (NSError *)createAndStoreMyDidWithWalletHandle:(IndyHandle)walletHandle
                                            seed:(NSString *)seed
                                        outMyDid:(NSString **)myDid
                                     outMyVerkey:(NSString **)myVerkey {

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *did = nil;
    __block NSString *verKey = nil;

    NSString *myDidJson = (seed) ? [NSString stringWithFormat:@"{\"seed\":\"%@\"}", seed] : @"{}";

    [IndyDid createAndStoreMyDid:myDidJson
                    walletHandle:walletHandle
                      completion:^(NSError *error, NSString *blockDid, NSString *blockVerKey) {
                          err = error;
                          did = blockDid;
                          verKey = blockVerKey;

                          [completionExpectation fulfill];
                      }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    if (myDid) {*myDid = did;}
    if (myVerkey) {*myVerkey = verKey;}

    return err;
}

- (void)createAndStoreAndPublishDidWithWalletHandle:(IndyHandle)walletHandle
                                         poolHandle:(IndyHandle)poolHandle
                                                did:(NSString **)did
                                             verkey:(NSString **)verkey {
    // 1. Obtain trustee did
    NSString *trusteeDid = nil;
    NSError *ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                             seed:[TestUtils trusteeSeed]
                                                                         outMyDid:&trusteeDid
                                                                      outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed for trustee");
    NSLog(@"trusteeDid: %@", trusteeDid);

    // 2. Obtain did
    __block NSString *tempDid;
    __block NSString *tempVerkey;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:@"{}"
                                                        outMyDid:&tempDid
                                                     outMyVerkey:&tempVerkey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed for myDid");

    // 3. Bulld nym request
    NSString *nymRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:tempDid
                                                                 verkey:tempVerkey
                                                                  alias:nil
                                                                   role:@"TRUSTEE"
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequest() failed");

    // 4. Sign and Submit nym request
    NSString *nymResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest
                                                           outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");

    if (did) {*did = tempDid;}
    if (verkey) {*verkey = tempVerkey;}
}


- (NSError *)storeTheirDidWithWalletHandle:(IndyHandle)walletHandle
                              identityJson:(NSString *)identityJson {

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyDid storeTheirDid:identityJson
              walletHandle:walletHandle
                completion:^(NSError *error) {
                    err = error;
                    [completionExpectation fulfill];
                }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    return err;
}

- (NSError *)storeTheirDidFromPartsWithWalletHandle:(IndyHandle)walletHandle
                                           theirDid:(NSString *)theirDid
                                        theirVerkey:(NSString *)theirVerkey {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    NSString *theirIdentityJson = [NSString stringWithFormat:@"{"
                                                                     "\"did\":\"%@\","
                                                                     "\"verkey\":\"%@\"}", theirDid, theirVerkey];

    [IndyDid storeTheirDid:theirIdentityJson
              walletHandle:walletHandle
                completion:^(NSError *error) {
                    err = error;
                    [completionExpectation fulfill];
                }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    return err;
}

- (NSError *)replaceKeysStartForDid:(NSString *)did
                       identityJson:(NSString *)identityJson
                       walletHandle:(IndyHandle)walletHandle
                        outMyVerKey:(NSString **)myVerKey {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *verkey;

    [IndyDid replaceKeysStartForDid:did
                       identityJson:identityJson
                       walletHandle:walletHandle
                         completion:^(NSError *error, NSString *blockVerkey) {
                             err = error;
                             verkey = blockVerkey;
                             [completionExpectation fulfill];
                         }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (myVerKey) {*myVerKey = verkey;}

    return err;
}

- (NSError *)replaceKeysApplyForDid:(NSString *)did
                       walletHandle:(IndyHandle)walletHandle {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyDid replaceKeysApplyForDid:did
                       walletHandle:walletHandle
                         completion:^(NSError *error) {
                             err = error;
                             [completionExpectation fulfill];
                         }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    return err;
}

- (NSError *)replaceKeysForDid:(NSString *)did
                  identityJson:(NSString *)identityJson
                  walletHandle:(IndyHandle)walletHandle
                    poolHandle:(IndyHandle)poolHandle
                   outMyVerKey:(NSString **)myVerKey {
    NSError *ret;

    NSString *verkey;

    ret = [self replaceKeysStartForDid:did
                          identityJson:identityJson
                          walletHandle:walletHandle
                           outMyVerKey:&verkey];

    if (ret.code != Success) {
        return ret;
    }

    NSString *nymRequest;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:did
                                                              targetDid:did
                                                                 verkey:verkey
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    if (ret.code != Success) {
        return ret;
    }

    NSString *nymResponce;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:did
                                                               requestJson:nymRequest
                                                           outResponseJson:&nymResponce];

    if (ret.code != Success) {
        return ret;
    }

    ret = [self replaceKeysApplyForDid:did
                          walletHandle:walletHandle];

    if (myVerKey) {*myVerKey = verkey;}

    return ret;
}


- (NSString *)createStoreAndPublishMyDidWithWalletHandle:(IndyHandle)walletHandle
                                              poolHandle:(IndyHandle)poolHandle {
    NSError *ret = nil;
    NSString *trusteeDid = nil;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:@"000000000000000000000000Trustee1"
                                                                outMyDid:&trusteeDid
                                                             outMyVerkey:nil];

    NSString *myDid = nil;
    NSString *myVerKey = nil;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:@"{}"
                                                        outMyDid:&myDid
                                                     outMyVerkey:&myVerKey];

    NSString *nymRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:myDid
                                                                 verkey:myVerKey
                                                                  alias:nil
                                                                   role:@"TRUSTEE"
                                                             outRequest:&nymRequest];

    NSString *nymResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest
                                                           outResponseJson:&nymResponse];

    return myDid;
}

- (NSError *)keyForDid:(NSString *)did
            poolHandle:(IndyHandle)poolHandle
          walletHandle:(IndyHandle)walletHandle
                   key:(NSString **)key {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *verkey;

    [IndyDid keyForDid:did
            poolHandle:poolHandle
          walletHandle:walletHandle
            completion:^(NSError *error, NSString *blockVerkey) {
                err = error;
                verkey = blockVerkey;
                [completionExpectation fulfill];
            }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (key) {*key = verkey;}

    return err;
}

- (NSError *)keyForLocalDid:(NSString *)did
               walletHandle:(IndyHandle)walletHandle
                        key:(NSString **)key {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *verkey;

    [IndyDid keyForLocalDid:did
               walletHandle:walletHandle
                 completion:^(NSError *error, NSString *blockVerkey) {
                     err = error;
                     verkey = blockVerkey;
                     [completionExpectation fulfill];
                 }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (key) {*key = verkey;}

    return err;
}

- (NSError *)setEndpointAddress:(NSString *)address
                   transportKey:(NSString *)transportKey
                         forDid:(NSString *)did
                   walletHandle:(IndyHandle)walletHandle {

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyDid setEndpointAddress:address
                   transportKey:transportKey
                         forDid:did
                   walletHandle:walletHandle
                     completion:^(NSError *error) {
                         err = error;
                         [completionExpectation fulfill];
                     }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    return err;
}

- (NSError *)getEndpointForDid:(NSString *)did
                  walletHandle:(IndyHandle)walletHandle
                    poolHandle:(IndyHandle)poolHandle
                       address:(NSString **)address
                  transportKey:(NSString **)transportKey {

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outAddress = nil;
    __block NSString *outTransportKey = nil;

    [IndyDid getEndpointForDid:did
                  walletHandle:walletHandle
                    poolHandle:poolHandle
                    completion:^(NSError *error, NSString *blockAddress, NSString *blockTransportKey) {
                        err = error;
                        outAddress = blockAddress;
                        outTransportKey = blockTransportKey;

                        [completionExpectation fulfill];
                    }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    if (address) {*address = outAddress;}
    if (transportKey) {*transportKey = outTransportKey;}

    return err;
}

- (NSError *)setMetadata:(NSString *)metadata
                  forDid:(NSString *)did
            walletHandle:(IndyHandle)walletHandle {

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyDid setMetadata:metadata
                  forDid:did
            walletHandle:walletHandle
              completion:^(NSError *error) {
                  err = error;
                  [completionExpectation fulfill];
              }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    return err;
}

- (NSError *)getMetadataForDid:(NSString *)did
                  walletHandle:(IndyHandle)walletHandle
                      metadata:(NSString **)metadata {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outMetadata;

    [IndyDid getMetadataForDid:did
                  walletHandle:walletHandle
                    completion:^(NSError *error, NSString *blockMetadata) {
                        err = error;
                        outMetadata = blockMetadata;
                        [completionExpectation fulfill];
                    }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (metadata) {*metadata = outMetadata;}

    return err;
}

- (NSError *)abbreviateVerkey:(NSString *)did
                   fullVerkey:(NSString *)fullVerkey
                       verkey:(NSString **)verkey {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outVerkey;

    [IndyDid abbreviateVerkey:did
                   fullVerkey:fullVerkey
                   completion:^(NSError *error, NSString *blockVerkey) {
                       err = error;
                       outVerkey = blockVerkey;
                       [completionExpectation fulfill];
                   }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (verkey) {*verkey = outVerkey;}

    return err;
}

- (NSError *)listMyDidsWithMeta:(IndyHandle)walletHandle
                       metadata:(NSString **)metadata {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outMetadata;

    [IndyDid listMyDidsWithMeta:walletHandle completion:^(NSError *error, NSString *metadata) {
        err = error;
        outMetadata = metadata;
        [completionExpectation fulfill];
    }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (metadata) {*metadata = outMetadata;}
    return err;
}

- (NSError *)qualifyDid:(NSString *)did
                 method:(NSString *)method
           walletHandle:(IndyHandle)walletHandle
       fullQualifiedDid:(NSString **)fullQualifiedDid
{
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyDid qualifyDid:did method:method walletHandle:walletHandle completion:^(NSError *error, NSString *did_) {
        err = error;
        if (fullQualifiedDid) *fullQualifiedDid = did_;
        [completionExpectation fulfill];
    }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    return err;
}

@end
