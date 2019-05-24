//
// Created by Evernym on 5/17/19.
// Copyright (c) 2019 Hyperledger. All rights reserved.
//

#import <Foundation/Foundation.h>
#import "IndyTypes.h"


@interface IndyCache : NSObject

/**
 Gets schema json data for specified schema id.
 If data is present inside of cache, cached data is returned.
 Otherwise data is fetched from the ledger and stored inside of cache for future use.

 EXPERIMENTAL

 @param poolHandle pool handle (created by open_pool_ledger).
 @param walletHandle wallet handle (created by open_wallet).
 @param submitterDid DID of the submitter stored in secured Wallet.
 @param id identifier of schema.
 @param optionsJson
  {
    noCache: (bool, optional, false by default) Skip usage of cache,
    noUpdate: (bool, optional, false by default) Use only cached data, do not try to update.
    noStore: (bool, optional, false by default) Skip storing fresh data if updated,
    minFresh: (int, optional, -1 by default) Return cached data if not older than this many seconds. -1 means do not check age.
  }

 @param completion Completion callback that returns error code and schema json:
  {
    id: identifier of schema
    attrNames: array of attribute name strings
    name: Schema's name string
    version: Schema's version string
    ver: Version of the Schema json
  }
 */
+ (void)getSchema:(IndyHandle)poolHandle
     walletHandle:(IndyHandle)walletHandle
     submitterDid:(NSString *)submitterDid
               id:(NSString *)id
      optionsJson:(NSString *)optionsJson
       completion:(void (^)(NSError *error, NSString *schemaJson))completion;

/**
 Gets credential definition json data for specified schema id.
 If data is present inside of cache, cached data is returned.
 Otherwise data is fetched from the ledger and stored inside of cache for future use.

 EXPERIMENTAL

 @param poolHandle pool handle (created by open_pool_ledger).
 @param walletHandle wallet handle (created by open_wallet).
 @param submitterDid DID of the submitter stored in secured Wallet.
 @param id identifier of credential definition.
 @param optionsJson
  {
    noCache: (bool, optional, false by default) Skip usage of cache,
    noUpdate: (bool, optional, false by default) Use only cached data, do not try to update.
    noStore: (bool, optional, false by default) Skip storing fresh data if updated,
    minFresh: (int, optional, -1 by default) Return cached data if not older than this many seconds. -1 means do not check age.
  }

 @param completion Completion callback that returns error code and credential definition json:
  {
    id: string - identifier of credential definition
    schemaId: string - identifier of stored in ledger schema
    type: string - type of the credential definition. CL is the only supported type now.
    tag: string - allows to distinct between credential definitions for the same issuer and schema
    value: Dictionary with Credential Definition's data: {
      primary: primary credential public key,
      Optional<revocation>: revocation credential public key
    },
    ver: Version of the Credential Definition json
  }
 */
+ (void)getCredDef:(IndyHandle)poolHandle
      walletHandle:(IndyHandle)walletHandle
      submitterDid:(NSString *)submitterDid
                id:(NSString *)id
       optionsJson:(NSString *)optionsJson
        completion:(void (^)(NSError *error, NSString *credDefJson))completion;

/**
 Purge schema cache

 EXPERIMENTAL

 @param walletHandle wallet handle (created by open_wallet).
 @param optionsJson
  {
    maxAge: (int, optional, -1 by default) Purge cached data if older than this many seconds. -1 means purge all.
  }

 @param completion Completion callback that returns error code
 */
+ (void)purgeSchemaCache:(IndyHandle)walletHandle
             optionsJson:(NSString *)optionsJson
              completion:(void (^)(NSError *error))completion;

/**
 Purge credential definition cache

 EXPERIMENTAL

 @param walletHandle wallet handle (created by open_wallet).
 @param optionsJson
  {
    maxAge: (int, optional, -1 by default) Purge cached data if older than this many seconds. -1 means purge all.
  }

 @param completion Completion callback that returns error code
 */
+ (void)purgeCredDefCache:(IndyHandle)walletHandle
              optionsJson:(NSString *)optionsJson
               completion:(void (^)(NSError *error))completion;

@end