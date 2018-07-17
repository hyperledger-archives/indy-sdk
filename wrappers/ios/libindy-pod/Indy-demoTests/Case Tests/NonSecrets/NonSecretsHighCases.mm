#import <XCTest/XCTest.h>
#import "TestUtils.h"
#import "NonSecretsUtils.h"

@interface NonSecretsHighCases : XCTestCase

@end

@implementation NonSecretsHighCases {
    IndyHandle walletHandle;
    NSError *ret;
}

- (void)setUp {
    [super setUp];
    [TestUtils cleanupStorage];

    ret = [[PoolUtils sharedInstance] setProtocolVersion:[TestUtils protocolVersion]];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::setProtocolVersion() failed!");

    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithHandle:&walletHandle];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
    [super tearDown];
}

// MARK: - Add Record

- (void)testAddWalletRecordWorks {
    ret = [[NonSecretsUtils sharedInstance] addRecordInWallet:walletHandle
                                                         type:[NonSecretsUtils type]
                                                           id:[NonSecretsUtils id1]
                                                        value:[NonSecretsUtils value1]
                                                     tagsJson:[NonSecretsUtils tags1]];
    XCTAssertEqual(ret.code, Success, @"NonSecretsUtils::addRecordInWallet() failed");
}

- (void)testAddWalletRecordWorksForDuplicate {
    ret = [[NonSecretsUtils sharedInstance] addRecordInWallet:walletHandle
                                                         type:[NonSecretsUtils type]
                                                           id:[NonSecretsUtils id1]
                                                        value:[NonSecretsUtils value1]
                                                     tagsJson:[NonSecretsUtils tags1]];
    XCTAssertEqual(ret.code, Success, @"NonSecretsUtils::addRecordInWallet() failed");

    ret = [[NonSecretsUtils sharedInstance] addRecordInWallet:walletHandle
                                                         type:[NonSecretsUtils type]
                                                           id:[NonSecretsUtils id1]
                                                        value:[NonSecretsUtils value1]
                                                     tagsJson:[NonSecretsUtils tags1]];
    XCTAssertEqual(ret.code, WalletItemAlreadyExists, @"NonSecretsUtils::addRecordInWallet() failed");
}


// MARK: - Update Record Value

- (void)testUpdateWalletRecordValueWorks {
    ret = [[NonSecretsUtils sharedInstance] addRecordInWallet:walletHandle
                                                         type:[NonSecretsUtils type]
                                                           id:[NonSecretsUtils id1]
                                                        value:[NonSecretsUtils value1]
                                                     tagsJson:[NonSecretsUtils tags1]];
    XCTAssertEqual(ret.code, Success, @"NonSecretsUtils::addRecordInWallet() failed");

    [NonSecretsUtils checkRecordField:walletHandle field:@"value" expectedValue:[NonSecretsUtils value1]];

    ret = [[NonSecretsUtils sharedInstance] updateRecordValueInWallet:walletHandle
                                                                 type:[NonSecretsUtils type]
                                                                   id:[NonSecretsUtils id1]
                                                                value:[NonSecretsUtils value2]];
    XCTAssertEqual(ret.code, Success, @"NonSecretsUtils::updateRecordValueInWallet() failed");

    [NonSecretsUtils checkRecordField:walletHandle field:@"value" expectedValue:[NonSecretsUtils value2]];
}

- (void)testUpdateWalletRecordValueWorksForNotFoundRecord {
    ret = [[NonSecretsUtils sharedInstance] updateRecordValueInWallet:walletHandle
                                                                 type:[NonSecretsUtils type]
                                                                   id:[NonSecretsUtils id1]
                                                                value:[NonSecretsUtils value2]];
    XCTAssertEqual(ret.code, WalletItemNotFound, @"NonSecretsUtils::updateRecordValueInWallet() failed");
}

// MARK: - Update Record Tags

- (void)testUpdateWalletRecordTagsWorks {
    ret = [[NonSecretsUtils sharedInstance] addRecordInWallet:walletHandle
                                                         type:[NonSecretsUtils type]
                                                           id:[NonSecretsUtils id1]
                                                        value:[NonSecretsUtils value1]
                                                     tagsJson:[NonSecretsUtils tags1]];
    XCTAssertEqual(ret.code, Success, @"NonSecretsUtils::addRecordInWallet() failed");

    [NonSecretsUtils checkRecordField:walletHandle field:@"tags" expectedValue:[NonSecretsUtils tags1]];

    ret = [[NonSecretsUtils sharedInstance] updateRecordTagsInWallet:walletHandle
                                                                type:[NonSecretsUtils type]
                                                                  id:[NonSecretsUtils id1]
                                                            tagsJson:[NonSecretsUtils tags2]];
    XCTAssertEqual(ret.code, Success, @"NonSecretsUtils::updateRecordTagsInWallet() failed");

    [NonSecretsUtils checkRecordField:walletHandle field:@"tags" expectedValue:[NonSecretsUtils tags2]];
}

- (void)testUpdateWalletRecordTagsWorksForNotFoundRecord {
    ret = [[NonSecretsUtils sharedInstance] updateRecordTagsInWallet:walletHandle
                                                                type:[NonSecretsUtils type]
                                                                  id:[NonSecretsUtils id1]
                                                            tagsJson:[NonSecretsUtils tags2]];
    XCTAssertEqual(ret.code, WalletItemNotFound, @"NonSecretsUtils::updateRecordTagsInWallet() failed");
}

// MARK: - Add Record Tags

- (void)testAddWalletRecordTagsWorks {
    ret = [[NonSecretsUtils sharedInstance] addRecordInWallet:walletHandle
                                                         type:[NonSecretsUtils type]
                                                           id:[NonSecretsUtils id1]
                                                        value:[NonSecretsUtils value1]
                                                     tagsJson:[NonSecretsUtils tagsEmpty]];
    XCTAssertEqual(ret.code, Success, @"NonSecretsUtils::addRecordInWallet() failed");

    ret = [[NonSecretsUtils sharedInstance] addRecordTagsInWallet:walletHandle
                                                             type:[NonSecretsUtils type]
                                                               id:[NonSecretsUtils id1]
                                                         tagsJson:[NonSecretsUtils tags1]];
    XCTAssertEqual(ret.code, Success, @"NonSecretsUtils::addRecordTagsInWallet() failed");

    [NonSecretsUtils checkRecordField:walletHandle field:@"tags" expectedValue:[NonSecretsUtils tags1]];
}

- (void)testAddWalletRecordTagsWorksForNotFoundRecord {
    ret = [[NonSecretsUtils sharedInstance] addRecordTagsInWallet:walletHandle
                                                             type:[NonSecretsUtils type]
                                                               id:[NonSecretsUtils id1]
                                                         tagsJson:[NonSecretsUtils tags2]];
    XCTAssertEqual(ret.code, WalletItemNotFound, @"NonSecretsUtils::addRecordTagsInWallet() failed");
}

// MARK: - Delete Record Tags

- (void)testDeleteWalletRecordTagsWorks {
    ret = [[NonSecretsUtils sharedInstance] addRecordInWallet:walletHandle
                                                         type:[NonSecretsUtils type]
                                                           id:[NonSecretsUtils id1]
                                                        value:[NonSecretsUtils value1]
                                                     tagsJson:[NonSecretsUtils tags1]];
    XCTAssertEqual(ret.code, Success, @"NonSecretsUtils::addRecordInWallet() failed");

    [NonSecretsUtils checkRecordField:walletHandle field:@"tags" expectedValue:[NonSecretsUtils tags1]];

    ret = [[NonSecretsUtils sharedInstance] deleteRecordTagsInWallet:walletHandle
                                                                type:[NonSecretsUtils type]
                                                                  id:[NonSecretsUtils id1]
                                                           tagsNames:@"[\"tagName1\"]"];
    XCTAssertEqual(ret.code, Success, @"NonSecretsUtils::deleteRecordTagsInWallet() failed");

    [NonSecretsUtils checkRecordField:walletHandle field:@"tags" expectedValue:@"{\"tagName2\":\"5\",\"tagName3\":\"12\"}"];
}

- (void)testDeleteWalletRecordTagsWorksForNotFoundRecord {
    ret = [[NonSecretsUtils sharedInstance] deleteRecordTagsInWallet:walletHandle
                                                                type:[NonSecretsUtils type]
                                                                  id:[NonSecretsUtils id1]
                                                           tagsNames:@"[\"tagName1\"]"];
    XCTAssertEqual(ret.code, WalletItemNotFound, @"NonSecretsUtils::deleteRecordTagsInWallet() failed");
}

// MARK: - Delete Record

- (void)testDeleteWalletRecordWorks {
    ret = [[NonSecretsUtils sharedInstance] addRecordInWallet:walletHandle
                                                         type:[NonSecretsUtils type]
                                                           id:[NonSecretsUtils id1]
                                                        value:[NonSecretsUtils value1]
                                                     tagsJson:[NonSecretsUtils tagsEmpty]];
    XCTAssertEqual(ret.code, Success, @"NonSecretsUtils::addRecordInWallet() failed");

    ret = [[NonSecretsUtils sharedInstance] deleteRecordInWallet:walletHandle
                                                            type:[NonSecretsUtils type]
                                                              id:[NonSecretsUtils id1]];
    XCTAssertEqual(ret.code, Success, @"NonSecretsUtils::deleteRecordInWallet() failed");
}

- (void)testDeleteWalletRecordWorksForNotFoundRecord {
    ret = [[NonSecretsUtils sharedInstance] deleteRecordInWallet:walletHandle
                                                            type:[NonSecretsUtils type]
                                                              id:[NonSecretsUtils id1]];
    XCTAssertEqual(ret.code, WalletItemNotFound, @"NonSecretsUtils::deleteRecordInWallet() failed");
}

// MARK: - Get Record

- (void)testGetWalletRecordWorks {
    ret = [[NonSecretsUtils sharedInstance] addRecordInWallet:walletHandle
                                                         type:[NonSecretsUtils type]
                                                           id:[NonSecretsUtils id1]
                                                        value:[NonSecretsUtils value1]
                                                     tagsJson:[NonSecretsUtils tags1]];
    XCTAssertEqual(ret.code, Success, @"NonSecretsUtils::addRecordInWallet() failed");

    NSString *recordJson;
    ret = [[NonSecretsUtils sharedInstance] getRecordFromWallet:walletHandle
                                                           type:[NonSecretsUtils type]
                                                             id:[NonSecretsUtils id1]
                                                    optionsJson:@"{}"
                                                     recordJson:&recordJson];
    XCTAssertEqual(ret.code, Success, @"NonSecretsUtils::getRecordFromWallet() failed");
    NSDictionary *record = [NSDictionary fromString:recordJson];

    XCTAssertTrue([[NonSecretsUtils id1] isEqualToString:record[@"id"]]);
    XCTAssertTrue([[NonSecretsUtils value1] isEqualToString:record[@"value"]]);
    XCTAssertTrue([record[@"type"] isEqual:[NSNull null]]);
    XCTAssertTrue([record[@"tags"] isEqual:[NSNull null]]);
}

- (void)testGetWalletRecordWorksForAllFields {
    ret = [[NonSecretsUtils sharedInstance] addRecordInWallet:walletHandle
                                                         type:[NonSecretsUtils type]
                                                           id:[NonSecretsUtils id1]
                                                        value:[NonSecretsUtils value1]
                                                     tagsJson:[NonSecretsUtils tags1]];
    XCTAssertEqual(ret.code, Success, @"NonSecretsUtils::addRecordInWallet() failed");

    NSString *recordJson;
    ret = [[NonSecretsUtils sharedInstance] getRecordFromWallet:walletHandle
                                                           type:[NonSecretsUtils type]
                                                             id:[NonSecretsUtils id1]
                                                    optionsJson:@"{\"retrieveType\":true, \"retrieveValue\":true, \"retrieveTags\":true}"
                                                     recordJson:&recordJson];
    XCTAssertEqual(ret.code, Success, @"NonSecretsUtils::getRecordFromWallet() failed");
    NSDictionary *record = [NSDictionary fromString:recordJson];

    XCTAssertTrue([[NonSecretsUtils id1] isEqualToString:record[@"id"]]);
    XCTAssertTrue([[NonSecretsUtils value1] isEqualToString:record[@"value"]]);
    XCTAssertTrue([[NonSecretsUtils type] isEqualToString:record[@"type"]]);
    XCTAssertTrue([[NSDictionary fromString:[NonSecretsUtils tags1]] isEqualToDictionary:record[@"tags"]]);
}

- (void)testGetWalletRecordWorksForNotFoundRecord {
    ret = [[NonSecretsUtils sharedInstance] getRecordFromWallet:walletHandle
                                                           type:[NonSecretsUtils type]
                                                             id:[NonSecretsUtils id1]
                                                    optionsJson:@"{}"
                                                     recordJson:nil];
    XCTAssertEqual(ret.code, WalletItemNotFound, @"NonSecretsUtils::getRecordFromWallet() failed");
}


// MARK: - Search

- (void)testSearchWorks {
    ret = [[NonSecretsUtils sharedInstance] addRecordInWallet:walletHandle
                                                         type:[NonSecretsUtils type]
                                                           id:[NonSecretsUtils id1]
                                                        value:[NonSecretsUtils value1]
                                                     tagsJson:[NonSecretsUtils tagsEmpty]];
    XCTAssertEqual(ret.code, Success, @"NonSecretsUtils::addRecordInWallet() failed");

    IndyHandle searchHandle;
    ret = [[NonSecretsUtils sharedInstance] openSearchInWallet:walletHandle
                                                          type:[NonSecretsUtils type]
                                                     queryJson:[NonSecretsUtils queryEmpty]
                                                   optionsJson:[NonSecretsUtils optionsEmpty]
                                                     outHandle:&searchHandle];
    XCTAssertEqual(ret.code, Success, @"NonSecretsUtils::openSearchInWallet() failed");

    NSString *walletRecords;
    ret = [[NonSecretsUtils sharedInstance] fetchNextRecordsFromSearch:searchHandle
                                                          walletHandle:walletHandle
                                                                 count:@(1)
                                                           recordsJson:&walletRecords];
    XCTAssertEqual(ret.code, Success, @"NonSecretsUtils::fetchNextRecordsFromSearch() failed");

    NSArray *records = [NSDictionary fromString:walletRecords][@"records"];
    XCTAssertEqual(1, [records count]);

    NSDictionary *record = records[0];
    XCTAssertTrue([[NonSecretsUtils id1] isEqualToString:record[@"id"]]);
    XCTAssertTrue([[NonSecretsUtils value1] isEqualToString:record[@"value"]]);
    XCTAssertTrue([record[@"type"] isEqual:[NSNull null]]);
    XCTAssertTrue([record[@"tags"] isEqual:[NSNull null]]);

    ret = [[NonSecretsUtils sharedInstance] closeSearchWithHandle:searchHandle];
    XCTAssertEqual(ret.code, Success, @"NonSecretsUtils::closeSearchWithHandle() failed");
}


@end

