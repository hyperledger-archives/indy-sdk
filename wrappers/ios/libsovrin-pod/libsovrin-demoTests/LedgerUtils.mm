//
//  LedgerUtils.m
//  libsovrin-demo
//
//  Created by Anastasia Tarasova on 05.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import "LedgerUtils.h"
#import <libsovrin/libsovrin.h>
#import "TestUtils.h"
#import "WalletUtils.h"

@implementation LedgerUtils

+ (LedgerUtils *)sharedInstance
{
    static LedgerUtils *instance = nil;
    static dispatch_once_t dispatch_once_block;
    
    dispatch_once(&dispatch_once_block, ^{
        instance = [LedgerUtils new];
    });
    
    return instance;
}


- (NSError *)signAndSubmitRequestWithPoolHandle:(SovrinHandle)poolHandle
                                   walletHandle:(SovrinHandle)walletHandle
                                   submitterDid:(NSString *)submitterDid
                                    requestJson:(NSString *)requestJson
                                outResponseJson:(NSString **)responseJson
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    __block NSString *outJson = nil;
    NSError *ret;

    ret = [SovrinLedger signAndSubmitRequest:walletHandle
                                  poolHandle:poolHandle
                                submitterDID:submitterDid
                                 requestJSON:requestJson
                                  completion:^(NSError* error, NSString *resultJson)
    {
        err = error;
        outJson = resultJson;
        [completionExpectation fulfill];
    }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    *responseJson = outJson;
    
    return err;
}

// MARK: Build nym request

- (NSError *) buildNymRequestWithSubmitterDid:(NSString *)submitterDid
                                    targetDid:(NSString *)targetDid
                                       verkey:(NSString *)verkey
                                         data:(NSString *)data
                                         role:(NSString *)role
                                   outRequest:(NSString **)resultJson;
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    __block NSString *outJson = nil;
    NSError *ret;
    
    NSString *xrefStr = (xref) ? xref : @"";
    NSString *dataStr = (data) ? data : @"";
    NSString *roleStr = (xref) ? role : @"";
    
    ret = [SovrinLedger buildNymRequest:submitterDid
                              targetDID:targetDid
                                 verkey:verkey
                                   xref:xrefStr
                                   data:dataStr
                                   role:roleStr
                             completion:^(NSError *error, NSString *json)
           {
               err = error;
               outJson = json;
               [completionExpectation fulfill];
           }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils longTimeout]];
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    *resultJson = outJson;
    
    return err;
}


- (NSError *) buildGetNymRequestWithSubmitterDid:(NSString *)submitterDid
                                       targetDid:(NSString *)targetDid
                                      outRequest:(NSString **)requestJson;
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    __block NSString *outJson = nil;
    NSError *ret;
    
    ret = [SovrinLedger buildGetNymRequest:submitterDid
                                 targetDID:targetDid
                                completion:^(NSError* error, NSString* json)
    {
        err = error;
        outJson = json;
        [completionExpectation fulfill];
    }];
    
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils longTimeout]];
    
    *requestJson = outJson;
    
    return err;
}

// MARK: Build Attribute request

- (NSError *)buildAttribRequest:(NSString *)submitterDid
                      targetDid:(NSString *)targetDid
                           hash:(NSString *)hash
                            raw:(NSString *)raw
                            enc:(NSString *)enc
                    resultJson:(NSString **)resultJson
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    __block NSString *outJson = nil;
    NSError *ret;
    
    NSString* hashStr = (hash) ? hash : @"";
    NSString* rawStr = (raw) ? raw : @"";
    NSString* encStr = (enc) ? enc : @"";
    
    ret = [SovrinLedger buildAttribRequest:submitterDid
                                 targetDID:targetDid
                                      hash:hashStr
                                       raw:rawStr
                                       enc:encStr
                                completion:^(NSError* error, NSString* requestJson)
           {
               err = error;
               outJson = requestJson;
               [completionExpectation fulfill];
           }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils longTimeout]];
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    *resultJson = outJson;
    return err;
}


- (NSError *)buildGetAttribRequest:(NSString *)submitterDid
                         targetDid:(NSString *)targetDid
                              data:(NSString *)data
                        resultJson:(NSString **)resultJson
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    __block NSString *outRequest = nil;
    NSError *ret;
    
    ret = [SovrinLedger buildGetAttribRequest:submitterDid
                                    targetDID:targetDid
                                         data:data
                                   completion:^(NSError* error, NSString* request)
    {
        err = error;
        outRequest = request;
        [completionExpectation fulfill];
    }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils longTimeout]];
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    *resultJson = outRequest;
    return err;
}
// MARK: Build schema request

- (NSError *)buildSchemaRequest:(NSString *)submitterDid
                           data:(NSString *)data
                     resultJson:(NSString **)resultJson
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    __block NSString *result = nil;
    NSError *ret;
    
    ret = [SovrinLedger buildSchemaRequest:submitterDid
                                      data:data
                                completion:^(NSError* error, NSString* request)
           {
               err = error;
               result = request;
               [completionExpectation fulfill];
           }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils longTimeout]];
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    *resultJson = result;
    return err;
}

- (NSError *)buildGetSchemaRequest:(NSString *)submitterDid
                              dest:(NSString *)dest
                              data:(NSString *)data
                        resultJson:(NSString **)resultJson
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    __block NSString *result = nil;
    NSError *ret;
    
    
    ret = [SovrinLedger buildGetSchemaRequest:submitterDid
                                         dest:dest
                                         data:data
                                   completion:^(NSError* error, NSString* request)
           {
               err = error;
               result = request;
               [completionExpectation fulfill];
           }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils longTimeout]];
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    *resultJson = result;
    return err;
}

// MARK: Build Node request

- (NSError *)buildNodeRequest:(NSString *) submitterDid
                    targetDid:(NSString *) targetDid
                         data:(NSString *) data
                   resultJson:(NSString **) resultJson
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    __block NSString *result = nil;
    NSError *ret;
    
    ret = [SovrinLedger buildNodeRequest:submitterDid
                               targetDid:targetDid
                                    data:data
                              completion:^(NSError* error, NSString* request)
           {
               err = error;
               result = request;
               [completionExpectation fulfill];
           }];
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils longTimeout]];
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    *resultJson = result;
    return err;
}

// MARK: Build claim definition txn

- (NSError *)buildClaimDefTxn:(NSString *) submitterDid
                         xref:(NSString *) xref
                signatureType:(NSString *) signatureType
                         data:(NSString *) data
                   resultJson:(NSString**) resultJson
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    __block NSString *result = nil;
    NSError *ret;
    
    ret = [SovrinLedger buildClaimDefTxn:submitterDid
                                    xref:xref
                           signatureType:signatureType
                                    data:data
                              completion:^(NSError* error, NSString* request)
           {
               err = error;
               result = request;
               [completionExpectation fulfill];
           }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils longTimeout]];
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    *resultJson = result;
    return err;
}

- (NSError *)buildGetClaimDefTxn:(NSString *) submitterDid
                            xref:(NSString *) xref
                   signatureType:(NSString *) signatureType
                          origin:(NSString *) origin
                      resultJson:(NSString**) resultJson
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    __block NSString *result = nil;
    NSError *ret;
    
    ret = [SovrinLedger buildGetClaimDefTxn:submitterDid
                                       xref:xref
                              signatureType:signatureType
                                     origin:origin
                                 completion:^(NSError* error, NSString* request)
           {
               err = error;
               result = request;
               [completionExpectation fulfill];
           }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils longTimeout]];
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    *resultJson = result;
    return err;
}

@end
