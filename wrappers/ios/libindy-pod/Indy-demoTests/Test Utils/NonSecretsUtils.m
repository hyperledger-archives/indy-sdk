#import "NonSecretsUtils.h"
#import "TestUtils.h"

@implementation NonSecretsUtils

+ (NonSecretsUtils *)sharedInstance {
    static NonSecretsUtils *instance = nil;
    static dispatch_once_t dispatch_once_block;

    dispatch_once(&dispatch_once_block, ^{
        instance = [NonSecretsUtils new];
    });

    return instance;
}

- (NSError *)addRecordInWallet:(IndyHandle)walletHandle
                          type:(NSString *)type
                            id:(NSString *)id
                         value:(NSString *)value
                      tagsJson:(NSString *)tagsJson {

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyNonSecrets addRecordInWallet:walletHandle
                                 type:type
                                   id:id
                                value:value
                             tagsJson:tagsJson
                           completion:^(NSError *error) {
                               err = error;
                               [completionExpectation fulfill];
                           }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    return err;
}

- (NSError *)updateRecordValueInWallet:(IndyHandle)walletHandle
                                  type:(NSString *)type
                                    id:(NSString *)id
                                 value:(NSString *)value {

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyNonSecrets updateRecordValueInWallet:walletHandle
                                         type:type
                                           id:id
                                        value:value
                                   completion:^(NSError *error) {
                                       err = error;
                                       [completionExpectation fulfill];
                                   }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    return err;
}

- (NSError *)updateRecordTagsInWallet:(IndyHandle)walletHandle
                                 type:(NSString *)type
                                   id:(NSString *)id
                             tagsJson:(NSString *)tagsJson {

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyNonSecrets updateRecordTagsInWallet:walletHandle
                                        type:type
                                          id:id
                                    tagsJson:tagsJson
                                  completion:^(NSError *error) {
                                      err = error;
                                      [completionExpectation fulfill];
                                  }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    return err;
}

- (NSError *)addRecordTagsInWallet:(IndyHandle)walletHandle
                              type:(NSString *)type
                                id:(NSString *)id
                          tagsJson:(NSString *)tagsJson {

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyNonSecrets addRecordTagsInWallet:walletHandle
                                     type:type
                                       id:id
                                 tagsJson:tagsJson
                               completion:^(NSError *error) {
                                   err = error;
                                   [completionExpectation fulfill];
                               }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    return err;
}

- (NSError *)deleteRecordTagsInWallet:(IndyHandle)walletHandle
                                 type:(NSString *)type
                                   id:(NSString *)id
                            tagsNames:(NSString *)tagsNames {

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyNonSecrets deleteRecordTagsInWallet:walletHandle
                                        type:type
                                          id:id
                                   tagsNames:tagsNames
                                  completion:^(NSError *error) {
                                      err = error;
                                      [completionExpectation fulfill];
                                  }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    return err;
}

- (NSError *)deleteRecordInWallet:(IndyHandle)walletHandle
                             type:(NSString *)type
                               id:(NSString *)id {

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyNonSecrets deleteRecordInWallet:walletHandle
                                    type:type
                                      id:id
                              completion:^(NSError *error) {
                                  err = error;
                                  [completionExpectation fulfill];
                              }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    return err;
}

- (NSError *)getRecordFromWallet:(IndyHandle)walletHandle
                            type:(NSString *)type
                              id:(NSString *)id
                     optionsJson:(NSString *)optionsJson
                      recordJson:(NSString **)recordJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyNonSecrets getRecordFromWallet:walletHandle
                                   type:type
                                     id:id
                            optionsJson:optionsJson
                             completion:^(NSError *error, NSString *record) {
                                 err = error;
                                 if (recordJson) *recordJson = record;
                                 [completionExpectation fulfill];
                             }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    return err;
}

- (NSError *)openSearchInWallet:(IndyHandle)walletHandle
                           type:(NSString *)type
                      queryJson:(NSString *)queryJson
                    optionsJson:(NSString *)optionsJson
                      outHandle:(IndyHandle *)handle {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyNonSecrets openSearchInWallet:walletHandle
                                  type:type
                             queryJson:queryJson
                           optionsJson:optionsJson
                            completion:^(NSError *error, IndyHandle h) {
                                err = error;
                                if (handle) *handle = h;
                                [completionExpectation fulfill];
                            }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    return err;
}

- (NSError *)fetchNextRecordsFromSearch:(IndyHandle)searchHandle
                           walletHandle:(IndyHandle)walletHandle
                                  count:(NSNumber *)count
                            recordsJson:(NSString **)recordsJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyNonSecrets fetchNextRecordsFromSearch:searchHandle
                                  walletHandle:walletHandle
                                         count:count
                                    completion:^(NSError *error, NSString *records) {
                                        err = error;
                                        if (recordsJson) *recordsJson = records;
                                        [completionExpectation fulfill];
                                    }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    return err;
}

- (NSError *)closeSearchWithHandle:(IndyHandle)searchHandle {

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyNonSecrets closeSearchWithHandle:searchHandle
                               completion:^(NSError *error) {
                                   err = error;
                                   [completionExpectation fulfill];
                               }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    return err;
}

+ (NSString *)type {
    return @"TestType";
}

+ (NSString *)id1 {
    return @"RecordId1";
}

+ (NSString *)id2 {
    return @"RecordId2";
}

+ (NSString *)id3 {
    return @"RecordId3";
}

+ (NSString *)value1 {
    return @"RecordValue1";
}

+ (NSString *)value2 {
    return @"RecordValue2";
}

+ (NSString *)value3 {
    return @"RecordValue3";
}

+ (NSString *)tagsEmpty {
    return @"{}";
}

+ (NSString *)optionsEmpty {
    return @"{}";
}

+ (NSString *)optionsFull {
    return @"{\"retrieveType\":true, \"retrieveValue\":true, \"retrieveTags\":true}";
}

+ (NSString *)queryEmpty {
    return @"{}";
}

+ (NSString *)tags1 {
    return @"{\"tagName1\":\"str1\",\"tagName2\":\"5\",\"tagName3\":\"12\"}";
}

+ (NSString *)tags2 {
    return @"{\"tagName1\":\"str2\",\"tagName2\":\"pre_str3\",\"tagName3\":\"2\"}";
}

+ (NSString *)tags3 {
    return @"{\"tagName1\":\"str1\",\"tagName2\":\"str2\",\"tagName3\":\"str3\"}";
}

+ (void)checkRecordField:(IndyHandle)walletHandle
                   field:(NSString *)field
           expectedValue:(NSString *)expectedValue {
    NSString *recordJson;
    NSError *ret = [[NonSecretsUtils sharedInstance] getRecordFromWallet:walletHandle
                                                                    type:[NonSecretsUtils type]
                                                                      id:[NonSecretsUtils id1]
                                                             optionsJson:[NonSecretsUtils optionsFull]
                                                              recordJson:&recordJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreCredentialDef() failed!");

    NSDictionary *record = [NSDictionary fromString:recordJson];

    if ([field isEqualToString:@"value"]) {
        XCTAssertTrue([expectedValue isEqualToString:record[@"value"]]);
    } else if ([field isEqualToString:@"tags"]) {
        NSDictionary *expectedTags = [NSDictionary fromString:expectedValue];
        XCTAssertTrue([expectedTags isEqualToDictionary:record[@"tags"]]);
    } else {
        XCTAssertTrue(NO);
    }
}


@end