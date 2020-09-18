//
// Created by Evernym on 5/16/18.
// Copyright (c) 2018 Hyperledger. All rights reserved.
//

#import <Foundation/Foundation.h>
#import "IndyTypes.h"


@interface IndyNonSecrets : NSObject

/**
 Create a new non-secret record in the wallet
 
 @param walletHandle wallet handle returned by IndyWallet::openWalletWithName.
 @param type allows to separate different record types collections
 @param id the id of record
 @param value the value of record
 @paran tagsJson the record tags used for search and storing meta information as json:
   {
     "tagName1": <str>, // string tag (will be stored encrypted)
     "tagName2": <str>, // string tag (will be stored encrypted)
     "~tagName3": <str>, // string tag (will be stored un-encrypted)
     "~tagName4": <str>, // string tag (will be stored un-encrypted)
   }
   Note that null means no tags
   If tag name starts with "~" the tag will be stored un-encrypted that will allow
   usage of this tag in complex search queries (comparison, predicates)
   Encrypted tags can be searched only for exact matching
 @param completion Completion callback that returns error code.
*/
+ (void)addRecordInWallet:(IndyHandle)walletHandle
                     type:(NSString *)type
                       id:(NSString *)id
                    value:(NSString *)value
                 tagsJson:(NSString *)tagsJson
               completion:(void (^)(NSError *error))completion;

/**
 Update a non-secret wallet record value
 
 @param walletHandle wallet handle returned by IndyWallet::openWalletWithName.
 @param type allows to separate different record types collections
 @param id the id of record
 @param value the value of record
 @param completion Completion callback that returns error code.
*/
+ (void)updateRecordValueInWallet:(IndyHandle)walletHandle
                             type:(NSString *)type
                               id:(NSString *)id
                            value:(NSString *)value
                       completion:(void (^)(NSError *error))completion;

/**
 Update a non-secret wallet record tags
 
 @param walletHandle wallet handle returned by IndyWallet::openWalletWithName.
 @param type allows to separate different record types collections
 @param id the id of record
 @paran tagsJson the record tags used for search and storing meta information as json:
   {
     "tagName1": <str>, // string tag (will be stored encrypted)
     "tagName2": <str>, // string tag (will be stored encrypted)
     "~tagName3": <str>, // string tag (will be stored un-encrypted)
     "~tagName4": <str>, // string tag (will be stored un-encrypted)
   }
   Note that null means no tags
   If tag name starts with "~" the tag will be stored un-encrypted that will allow
   usage of this tag in complex search queries (comparison, predicates)
   Encrypted tags can be searched only for exact matching
 @param completion Completion callback that returns error code.
*/
+ (void)updateRecordTagsInWallet:(IndyHandle)walletHandle
                            type:(NSString *)type
                              id:(NSString *)id
                        tagsJson:(NSString *)tagsJson
                      completion:(void (^)(NSError *error))completion;

/**
 Add new tags to the wallet record
 
 @param walletHandle wallet handle returned by IndyWallet::openWalletWithName.
 @param type allows to separate different record types collections
 @param id the id of record
 @paran tagsJson the record tags used for search and storing meta information as json:
   {
     "tagName1": <str>, // string tag (will be stored encrypted)
     "tagName2": <str>, // string tag (will be stored encrypted)
     "~tagName3": <str>, // string tag (will be stored un-encrypted)
     "~tagName4": <str>, // string tag (will be stored un-encrypted)
   }
   Note that null means no tags
   If tag name starts with "~" the tag will be stored un-encrypted that will allow
   usage of this tag in complex search queries (comparison, predicates)
   Encrypted tags can be searched only for exact matching
 @param completion Completion callback that returns error code.
*/
+ (void)addRecordTagsInWallet:(IndyHandle)walletHandle
                         type:(NSString *)type
                           id:(NSString *)id
                     tagsJson:(NSString *)tagsJson
                   completion:(void (^)(NSError *error))completion;


/**
 Delete tags from the wallet record
 
 @param walletHandle wallet handle returned by IndyWallet::openWalletWithName.
 @param type allows to separate different record types collections
 @param id the id of record
 @paran tag_names_json: the list of tag names to remove from the record as json array: ["tagName1", "tagName2", ...]
 @param completion Completion callback that returns error code.
*/
+(void)deleteRecordTagsInWallet:(IndyHandle)walletHandle
                            type:(NSString *)type
                              id:(NSString *)id
                       tagsNames:(NSString *)tagsNames
                      completion:(void (^)(NSError *error))completion;

/**
 Delete an existing wallet record in the wallet
 
 @param walletHandle wallet handle returned by IndyWallet::openWalletWithName.
 @param type allows to separate different record types collections
 @param id the id of record
 @paran tag_names_json: the list of tag names to remove from the record as json array: ["tagName1", "tagName2", ...]
 @param completion Completion callback that returns error code.
*/
+ (void)deleteRecordInWallet:(IndyHandle)walletHandle
                        type:(NSString *)type
                          id:(NSString *)id
                  completion:(void (^)(NSError *error))completion;

/**
 Get an wallet record by id
 
 @param walletHandle wallet handle returned by IndyWallet::openWalletWithName.
 @param type allows to separate different record types collections
 @param id the id of record
 @param optionsJson:
  {
    retrieveType: (optional, false by default) Retrieve record type,
    retrieveValue: (optional, true by default) Retrieve record value,
    retrieveTags: (optional, false by default) Retrieve record tags
  } 
 
 @param completion Completion callback that returns error code and wallet record:
 {
   id: "Some id",
   type: "Some type", // present only if retrieveType set to true
   value: "Some value", // present only if retrieveValue set to true
   tags: <tags json>, // present only if retrieveTags set to true
 }.
 */
+ (void)getRecordFromWallet:(IndyHandle)walletHandle
                       type:(NSString *)type
                         id:(NSString *)id
                optionsJson:(NSString *)optionsJson
                 completion:(void (^)(NSError *error, NSString *recordJson))completion;

/**
 Search for wallet records.

 Note instead of immediately returning of fetched records
 this call returns wallet_search_handle that can be used later
 to fetch records by small batches (with fetchNextRecord).
 
 @param walletHandle wallet handle returned by IndyWallet::openWalletWithName.
 @param type allows to separate different record types collections
 @param queryJson MongoDB style query to wallet record tags:
  {
    "tagName": "tagValue",
    $or: {
      "tagName2": { $regex: 'pattern' },
      "tagName3": { $gte: '123' },
    },
  }
 @param optionsJson:
  {
    retrieveRecords: (optional, true by default) If false only "counts" will be calculated,
    retrieveTotalCount: (optional, false by default) Calculate total count,
    retrieveType: (optional, false by default) Retrieve record type,
    retrieveValue: (optional, true by default) Retrieve record value,
    retrieveTags: (optional, false by default) Retrieve record tags,
  }
 
 @param completion Completion callback that returns error code and searchHandle that can be used later
   to fetch records by small batches
 */
+ (void)openSearchInWallet:(IndyHandle)walletHandle
                      type:(NSString *)type
                 queryJson:(NSString *)queryJson
               optionsJson:(NSString *)optionsJson
                completion:(void (^)(NSError *error, IndyHandle searchHandle))completion;

/**
 Fetch next records for wallet search.

 Not if there are no records this call returns WalletNoRecords error.

 @param searchHandle wallet search handle (created by openSearchInWallet)
 @param walletHandle wallet handle returned by IndyWallet::openWalletWithName.
 @param count Count of records to fetch
 
 @param completion Completion callback that returns error code and wallet records json wallet records json:
 {
   totalCount: <str>, // present only if retrieveTotalCount set to true
   records: [{ // present only if retrieveRecords set to true
       id: "Some id",
       type: "Some type", // present only if retrieveType set to true
       value: "Some value", // present only if retrieveValue set to true
       tags: <tags json>, // present only if retrieveTags set to true
   }],
 }
 */
+ (void)fetchNextRecordsFromSearch:(IndyHandle)searchHandle
                      walletHandle:(IndyHandle)walletHandle
                             count:(NSNumber *)count
                        completion:(void (^)(NSError *error, NSString *recordsJson))completion;

/**
 Close wallet search (make search handle invalid)

 @param searchHandle wallet search handle (created by openSearchInWallet)

 @param completion Completion callback that returns error code
 */
+ (void)closeSearchWithHandle:(IndyHandle)searchHandle
         completion:(void (^)(NSError *error))completion;

@end
