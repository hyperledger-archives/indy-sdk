#import <XCTest/XCTest.h>

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import "TestUtils.h"
#import "PairwiseUtils.h"


@interface PairwiseHighCases : XCTestCase

@end

@implementation PairwiseHighCases {
    IndyHandle walletHandle;
    NSError *ret;
}

- (void)setUp {
    [super setUp];
    [TestUtils cleanupStorage];

    ret = [[PoolUtils sharedInstance] setProtocolVersion:[TestUtils protocolVersion]];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::setProtocolVersion() failed!");

    [[WalletUtils sharedInstance] createAndOpenWalletWithHandle:&walletHandle];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
    [super tearDown];
}

// MARK: - Create

- (void)testCreatePairwiseWorks {
    // 1. create and store my did
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:[TestUtils mySeed1]
                                                                outMyDid:&myDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create myDid!");

    // 2. Store their did
    ret = [[DidUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                   theirDid:[TestUtils trusteeDid]
                                                                theirVerkey:[TestUtils trusteeVerkey]];
    XCTAssertEqual(ret.code, Success, @"DidUtils::storeTheirDidWithWalletHandle() failed!");

    // 3. create pairwise
    ret = [[PairwiseUtils sharedInstance] createPairwiseForTheirDid:[TestUtils trusteeDid]
                                                          withMyDid:myDid
                                                           metadata:[TestUtils someMetadata]
                                                       walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"PairwiseUtils::createPairwiseForTheirDid() failed!");
}

- (void)testCreatePairwiseWorksForNotFoundMyDid {
    // 1. store their did
    ret = [[DidUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                   theirDid:[TestUtils trusteeDid]
                                                                theirVerkey:[TestUtils trusteeVerkey]];
    XCTAssertEqual(ret.code, Success, @"DidUtils::storeTheirDidWithWalletHandle() failed!");

    // 2. create pairwise
    ret = [[PairwiseUtils sharedInstance] createPairwiseForTheirDid:[TestUtils trusteeDid]
                                                          withMyDid:[TestUtils unknownDid]
                                                           metadata:[TestUtils someMetadata]
                                                       walletHandle:walletHandle];
    XCTAssertEqual(ret.code, WalletItemNotFound, @"PairwiseUtils::createPairwiseForTheirDid() returned wrong eror code!");
}

// MARK: - List pairwise

- (void)testListPairwiseWorks {
    // 1. create and store my did
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:[TestUtils mySeed1]
                                                                outMyDid:&myDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create myDid!");

    // 2. create and store their did
    ret = [[DidUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                   theirDid:[TestUtils trusteeDid]
                                                                theirVerkey:[TestUtils trusteeVerkey]];
    XCTAssertEqual(ret.code, Success, @"DidUtils::storeTheirDidWithWalletHandle() failed!");

    // 3. create pairwise
    ret = [[PairwiseUtils sharedInstance] createPairwiseForTheirDid:[TestUtils trusteeDid]
                                                          withMyDid:myDid
                                                           metadata:[TestUtils someMetadata]
                                                       walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"PairwiseUtils::createPairwiseForTheirDid() failed!");

    // 4. list pairwise
    NSString *pairwiseListJson;
    ret = [[PairwiseUtils sharedInstance] listPairwiseFromWallet:walletHandle
                                                 outPairwiseList:&pairwiseListJson];
    XCTAssertEqual(ret.code, Success, @"PairwiseUtils::listPairwiseFromWallet() failed!");

    NSArray *pairwiseList = (NSArray *) [NSDictionary fromString:pairwiseListJson];
    XCTAssertTrue([pairwiseList count] == 1, @"pairwiseList count != 1.");

    NSDictionary *expectedResult = @{@"my_did": myDid, @"their_did": [TestUtils trusteeDid]};

    XCTAssertTrue([pairwiseList contains:expectedResult], @"pairwiseList doesn't contain pair: %@", expectedResult);
}

- (void)testListPairwiseWorksForEmptyResult {
    // 1. list pairwise
    NSString *pairwiseListJson;
    ret = [[PairwiseUtils sharedInstance] listPairwiseFromWallet:walletHandle
                                                 outPairwiseList:&pairwiseListJson];
    XCTAssertEqual(ret.code, Success, @"PairwiseUtils::listPairwiseFromWallet() failed!");

    NSArray *pairwiseList = (NSArray *) [NSDictionary fromString:pairwiseListJson];
    XCTAssertTrue([pairwiseList count] == 0, @"pairwiseList count != 0.");
}

// MARK: - Pairwise exists

- (void)testPairwiseExistsWorks {
    // 1. create and store my did
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:[TestUtils mySeed1]
                                                                outMyDid:&myDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create myDid!");

    // 2. create and store their did
    ret = [[DidUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                   theirDid:[TestUtils trusteeDid]
                                                                theirVerkey:[TestUtils trusteeVerkey]];
    XCTAssertEqual(ret.code, Success, @"DidUtils::storeTheirDidWithWalletHandle() failed!");

    // 3. create pairwise
    ret = [[PairwiseUtils sharedInstance] createPairwiseForTheirDid:[TestUtils trusteeDid]
                                                          withMyDid:myDid
                                                           metadata:[TestUtils someMetadata]
                                                       walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"PairwiseUtils::createPairwiseForTheirDid() failed!");

    // 4. Check if pairwise exists
    BOOL exists;
    ret = [[PairwiseUtils sharedInstance] pairwiseExistsForDid:[TestUtils trusteeDid]
                                                  walletHandle:walletHandle
                                                     outExists:&exists];
    XCTAssertEqual(ret.code, Success, @"PairwiseUtils::pairwiseExistsForDid() failed!");
    XCTAssertTrue(exists, @"pairwise doesn't exist!");
}

- (void)testIsPairwiseExistsWorksForNotCreated {
    // 1. Check if pairwise exists
    BOOL exists = false;
    ret = [[PairwiseUtils sharedInstance] pairwiseExistsForDid:[TestUtils trusteeDid]
                                                  walletHandle:walletHandle
                                                     outExists:&exists];
    XCTAssertEqual(ret.code, Success, @"PairwiseUtils::pairwiseExistsForDid() failed!");
    XCTAssertFalse(exists, @"pairwise exists!");
}

// MARK: - Get pairwise

- (void)testGetPairwiseWorks {
    // 1. create and store my did
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:[TestUtils mySeed1]
                                                                outMyDid:&myDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create myDid!");

    // 2. create and store their did
    ret = [[DidUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                   theirDid:[TestUtils trusteeDid]
                                                                theirVerkey:[TestUtils trusteeVerkey]];
    XCTAssertEqual(ret.code, Success, @"DidUtils::storeTheirDidWithWalletHandle() failed!");

    // 3. create pairwise
    ret = [[PairwiseUtils sharedInstance] createPairwiseForTheirDid:[TestUtils trusteeDid]
                                                          withMyDid:myDid
                                                           metadata:[TestUtils someMetadata]
                                                       walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"PairwiseUtils::createPairwiseForTheirDid() failed!");

    // 4. get pairwise
    NSString *pairwiseInfoJson;
    ret = [[PairwiseUtils sharedInstance] getPairwiseForDid:[TestUtils trusteeDid]
                                               walletHandle:walletHandle
                                            outPairwiseJson:&pairwiseInfoJson];
    XCTAssertEqual(ret.code, Success, @"PairwiseUtils::getPairwiseForDid() failed!");

    NSDictionary *pairwiseInfo = [NSDictionary fromString:pairwiseInfoJson];

    NSDictionary *expectedResult = @{@"my_did": myDid, @"metadata": [TestUtils someMetadata]};

    XCTAssertTrue([pairwiseInfo contains:expectedResult], @"expectedResult is not contained in pairwiseInfoJson.");
}

- (void)testGetPairwiseWorksForNotCreatedPairwise {
    // 1. get pairwise

    NSString *pairwiseInfoJson;
    ret = [[PairwiseUtils sharedInstance] getPairwiseForDid:[TestUtils trusteeDid]
                                               walletHandle:walletHandle
                                            outPairwiseJson:&pairwiseInfoJson];
    XCTAssertEqual(ret.code, WalletItemNotFound, @"PairwiseUtils::getPairwiseForDid() returned wrong error code!");
}

// MARK: - Set pairwise metadata

- (void)testSetPairwiseMetadataWorks {
    // 1. create and store my did
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:[TestUtils mySeed1]
                                                                outMyDid:&myDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create myDid!");

    // 2. create and store their did
    ret = [[DidUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                   theirDid:[TestUtils trusteeDid]
                                                                theirVerkey:[TestUtils trusteeVerkey]];
    XCTAssertEqual(ret.code, Success, @"DidUtils::storeTheirDidWithWalletHandle() failed!");

    // 3. create pairwise
    ret = [[PairwiseUtils sharedInstance] createPairwiseForTheirDid:[TestUtils trusteeDid]
                                                          withMyDid:myDid
                                                           metadata:[TestUtils someMetadata]
                                                       walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"PairwiseUtils::createPairwiseForTheirDid() failed!");

    // 4. get pairwise info without metadata
    NSString *pairwiseInfoWithoutMetadataJson;
    ret = [[PairwiseUtils sharedInstance] getPairwiseForDid:[TestUtils trusteeDid]
                                               walletHandle:walletHandle
                                            outPairwiseJson:&pairwiseInfoWithoutMetadataJson];
    XCTAssertEqual(ret.code, Success, @"PairwiseUtils::getPairwiseForDid() failed!");

    NSDictionary *pairwiseInfoWithoutMetadata = [NSDictionary fromString:pairwiseInfoWithoutMetadataJson];

    NSDictionary *expectedResult = @{@"my_did": myDid};

    XCTAssertTrue([pairwiseInfoWithoutMetadata contains:expectedResult], @"expectedResult is not contained in pairwiseInfoWithoutMetadata.");

    // 5. Set pairwise metadata
    ret = [[PairwiseUtils sharedInstance] setPairwiseMetadata:[TestUtils someMetadata]
                                                  forTheirDid:[TestUtils trusteeDid]
                                                 walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"PairwiseUtils::setPairwiseMetadata() failed!");

    // 6. get pairwise info with metadata
    NSString *pairwiseInfoWithMetadataJson;
    ret = [[PairwiseUtils sharedInstance] getPairwiseForDid:[TestUtils trusteeDid]
                                               walletHandle:walletHandle
                                            outPairwiseJson:&pairwiseInfoWithMetadataJson];
    XCTAssertEqual(ret.code, Success, @"PairwiseUtils::getPairwiseForDid() failed!");

    NSDictionary *pairwiseInfoWithMetadata = [NSDictionary fromString:pairwiseInfoWithMetadataJson];

    expectedResult = @{@"my_did": myDid, @"metadata": [TestUtils someMetadata]};

    XCTAssertTrue([pairwiseInfoWithMetadata contains:expectedResult], @"expectedResult is not contained in pairwiseInfoWithMetadata.");
}

- (void)testSetPairwiseMetadataWorksForNotCreatedPairwise {
    // 1. Set pairwise metadata
    ret = [[PairwiseUtils sharedInstance] setPairwiseMetadata:[TestUtils someMetadata]
                                                  forTheirDid:[TestUtils trusteeDid]
                                                 walletHandle:walletHandle];
    XCTAssertEqual(ret.code, WalletItemNotFound, @"PairwiseUtils::setPairwiseMetadata() returned wrong error code!");
}

@end
