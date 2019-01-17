
#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>


@interface NonSecretsUtils : XCTestCase

+ (NonSecretsUtils *)sharedInstance;

- (NSError *)addRecordInWallet:(IndyHandle)walletHandle
                          type:(NSString *)type
                            id:(NSString *)id
                         value:(NSString *)value
                      tagsJson:(NSString *)tagsJson;

- (NSError *)updateRecordValueInWallet:(IndyHandle)walletHandle
                                  type:(NSString *)type
                                    id:(NSString *)id
                                 value:(NSString *)value;

- (NSError *)updateRecordTagsInWallet:(IndyHandle)walletHandle
                                 type:(NSString *)type
                                   id:(NSString *)id
                             tagsJson:(NSString *)tagsJson;

- (NSError *)addRecordTagsInWallet:(IndyHandle)walletHandle
                              type:(NSString *)type
                                id:(NSString *)id
                          tagsJson:(NSString *)tagsJson;

- (NSError *)deleteRecordTagsInWallet:(IndyHandle)walletHandle
                                 type:(NSString *)type
                                   id:(NSString *)id
                            tagsNames:(NSString *)tagsNames;

- (NSError *)deleteRecordInWallet:(IndyHandle)walletHandle
                             type:(NSString *)type
                               id:(NSString *)id;

- (NSError *)getRecordFromWallet:(IndyHandle)walletHandle
                            type:(NSString *)type
                              id:(NSString *)id
                     optionsJson:(NSString *)optionsJson
                      recordJson:(NSString **)recordJson;

- (NSError *)openSearchInWallet:(IndyHandle)walletHandle
                           type:(NSString *)type
                      queryJson:(NSString *)queryJson
                    optionsJson:(NSString *)optionsJson
                      outHandle:(IndyHandle *)handle;

- (NSError *)fetchNextRecordsFromSearch:(IndyHandle)searchHandle
                           walletHandle:(IndyHandle)walletHandle
                                  count:(NSNumber *)count
                            recordsJson:(NSString **)recordsJson;

- (NSError *)closeSearchWithHandle:(IndyHandle)searchHandle;

+ (NSString *)type;

+ (NSString *)id1;

+ (NSString *)id2;

+ (NSString *)id3;

+ (NSString *)value1;

+ (NSString *)value2;

+ (NSString *)value3;

+ (NSString *)tagsEmpty;

+ (NSString *)optionsEmpty;

+ (NSString *)optionsFull;

+ (NSString *)queryEmpty;

+ (NSString *)tags1;

+ (NSString *)tags2;

+ (NSString *)tags3;

+ (void)checkRecordField:(IndyHandle)walletHandle
                   field:(NSString *)field
           expectedValue:(NSString *)expectedValue;

@end