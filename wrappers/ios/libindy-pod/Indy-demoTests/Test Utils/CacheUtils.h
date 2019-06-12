//
//  CacheUtils.h
//  Indy-demo
//
// Created by Evernym on 5/17/19.
// Copyright (c) 2019 Hyperledger. All rights reserved.
//


#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>

@interface CacheUtils : XCTestCase

+ (CacheUtils *)sharedInstance;

- (NSError *)purgeSchemaCache:(IndyHandle)walletHandle
                  optionsJson:(NSString *)optionsJson;

- (NSError *)purgeCredDefCache:(IndyHandle)walletHandle
                   optionsJson:(NSString *)optionsJson;

- (NSError *)getSchema:(IndyHandle)poolHandle
          walletHandle:(IndyHandle)walletHandle
          submitterDid:(NSString *)submitterDid
                    id:(NSString *)id
           optionsJson:(NSString *)optionsJson
            schemaJson:(NSString **)schemaJson;

- (NSError *)getCredDef:(IndyHandle)poolHandle
           walletHandle:(IndyHandle)walletHandle
           submitterDid:(NSString *)submitterDid
                     id:(NSString *)id
            optionsJson:(NSString *)optionsJson
            credDefJson:(NSString **)credDefJson;

@end
