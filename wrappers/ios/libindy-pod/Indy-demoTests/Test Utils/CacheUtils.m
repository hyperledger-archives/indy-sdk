//
//  CacheUtils.m
//  Indy-demo
//
// Created by Evernym on 5/17/19.
// Copyright (c) 2019 Hyperledger. All rights reserved.
//

#import "CacheUtils.h"
#import <Indy/Indy.h>
#import "TestUtils.h"
#import "WalletUtils.h"

@implementation CacheUtils

+ (CacheUtils *)sharedInstance {
    static LedgerUtils *instance = nil;
    static dispatch_once_t dispatch_once_block;

    dispatch_once(&dispatch_once_block, ^{
        instance = [LedgerUtils new];
    });

    return instance;
}

- (NSError *)purgeSchemaCache:(IndyHandle)walletHandle
                  optionsJson:(NSString *)optionsJson {

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyCache purgeSchemaCache:walletHandle
                     optionsJson:optionsJson
                      completion:^(NSError *error) {
                          err = error;
                          [completionExpectation fulfill];
                      }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    return err;
}

- (NSError *)purgeCredDefCache:(IndyHandle)walletHandle
                   optionsJson:(NSString *)optionsJson {

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyCache purgeCredDefCache:walletHandle
                     optionsJson:optionsJson
                      completion:^(NSError *error) {
                          err = error;
                          [completionExpectation fulfill];
                      }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    return err;
}

- (NSError *)getSchema:(IndyHandle)poolHandle
          walletHandle:(IndyHandle)walletHandle
          submitterDid:(NSString *)submitterDid
                    id:(NSString *)id
           optionsJson:(NSString *)optionsJson
            schemaJson:(NSString **)schemaJson {

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyCache getSchema:poolHandle
            walletHandle:walletHandle
            submitterDid:submitterDid
                      id:id
             optionsJson:optionsJson
              completion:^[NSError *error, NSString *record {
                  err = error;
                  if (schemaJson) *schemaJson = record;
                  [completionExpectation fulfill];
              }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    return err;
}

- (NSError *)getCredDef:(IndyHandle)poolHandle
           walletHandle:(IndyHandle)walletHandle
           submitterDid:(NSString *)submitterDid
                     id:(NSString *)id
            optionsJson:(NSString *)optionsJson
            credDefJson:(NSString **)credDefJson {

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyCache getCredDef:poolHandle
             walletHandle:walletHandle
             submitterDid:submitterDid
                       id:id
              optionsJson:optionsJson
               completion:^[NSError *error, NSString *record {
                   err = error;
                   if (credDefJson) *credDefJson = record;
                   [completionExpectation fulfill];
               }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    return err;
}

@end