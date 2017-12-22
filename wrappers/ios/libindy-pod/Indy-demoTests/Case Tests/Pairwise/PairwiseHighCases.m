//
//  PairwiseHighCases.m
//  Indy-demo
//
//  Created by Anastasia Tarasova on 04/10/2017.
//  Copyright © 2017 Hyperledger. All rights reserved.
//

#import <XCTest/XCTest.h>

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import "TestUtils.h"
#import "PairwiseUtils.h"


@interface PairwiseHighCases : XCTestCase

@end

@implementation PairwiseHighCases

- (void)setUp {
    [super setUp];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

- (void)testExample {
    // This is an example of a functional test case.
    // Use XCTAssert and related functions to verify your tests produce the correct results.
}

- (void)testPerformanceExample {
    // This is an example of a performance test case.
    [self measureBlock:^{
        // Put the code you want to measure the time of here.
    }];
}

// MARK: - Create

- (void)testCreatePairwiseWorks
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. create and open wallet
    
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual( ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed!");
    
    // 2. create and store my did
    
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed1]
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create myDid!");
    
    // 3. create and store their did
    
    NSString *theirDid;
    NSString *theirVerkey;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed2]
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create theirDid!");
 
    // 4. Store their identity json
    
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}", theirDid, theirVerkey];
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual( ret.code, Success, @"DidUtils::storeTheirDidWithWalletHandle() failed!");
    
    // 5. create pairwise
    ret = [[PairwiseUtils sharedInstance] createPairwiseForTheirDid:theirDid
                                                          withMyDid:myDid
                                                           metadata:[TestUtils someMetadata]
                                                       walletHandle:walletHandle];
    XCTAssertEqual( ret.code, Success, @"PairwiseUtils::createPairwiseForTheirDid() failed!");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testCreatePairwiseWorksForEmptyMetadata
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. create and open wallet
    
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual( ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed!");
    
    // 2. create and store my did
    
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed1]
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create myDid!");
    
    // 3. create and store their did
    
    NSString *theirDid;
    NSString *theirVerkey;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed2]
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create theirDid!");
    
    // 4. Store their identity json
    
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}", theirDid, theirVerkey];
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual( ret.code, Success, @"DidUtils::storeTheirDidWithWalletHandle() failed!");
    
    // 5. create pairwise
    ret = [[PairwiseUtils sharedInstance] createPairwiseForTheirDid:theirDid
                                                          withMyDid:myDid
                                                           metadata:nil
                                                       walletHandle:walletHandle];
    XCTAssertEqual( ret.code, Success, @"PairwiseUtils::createPairwiseForTheirDid() failed!");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testCreatePairwiseWorksForNotFoundMyDid
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. create and open wallet
    
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual( ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed!");
    
    // 2. create and store their did
    
    NSString *theirDid;
    NSString *theirVerkey;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed2]
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create theirDid!");
    
    // 3. Store their identity json
    
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}", theirDid, theirVerkey];
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual( ret.code, Success, @"DidUtils::storeTheirDidWithWalletHandle() failed!");
    
    // 4. create pairwise
    ret = [[PairwiseUtils sharedInstance] createPairwiseForTheirDid:theirDid
                                                          withMyDid:[TestUtils unknownDid]
                                                           metadata:[TestUtils someMetadata]
                                                       walletHandle:walletHandle];
    XCTAssertEqual( ret.code, WalletNotFoundError, @"PairwiseUtils::createPairwiseForTheirDid() returned wrong eror code!");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testCreatePairwiseWorksForNotFoundTheirDid
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. create and open wallet
    
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual( ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed!");
    
    // 2. create and store my did
    
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed1]
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create myDid!");
    
    // 3. create pairwise
    ret = [[PairwiseUtils sharedInstance] createPairwiseForTheirDid:[TestUtils unknownDid]
                                                          withMyDid:myDid
                                                           metadata:[TestUtils someMetadata]
                                                       walletHandle:walletHandle];
    XCTAssertEqual( ret.code, WalletNotFoundError, @"PairwiseUtils::createPairwiseForTheirDid() failed!");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testCreatePairwiseWorksForInvalidHandle
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. create and open wallet
    
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual( ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed!");
    
    // 2. create and store my did
    
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed1]
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create myDid!");
    
    // 3. create and store their did
    
    NSString *theirDid;
    NSString *theirVerkey;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed2]
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create theirDid!");
    
    // 4. Store their identity json
    
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}", theirDid, theirVerkey];
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual( ret.code, Success, @"DidUtils::storeTheirDidWithWalletHandle() failed!");
    
    // 5. create pairwise
    IndyHandle invalidWalletHandle = walletHandle + 1;
    ret = [[PairwiseUtils sharedInstance] createPairwiseForTheirDid:theirDid
                                                          withMyDid:myDid
                                                           metadata:[TestUtils someMetadata]
                                                       walletHandle:invalidWalletHandle];
    XCTAssertEqual( ret.code, WalletInvalidHandle, @"PairwiseUtils::createPairwiseForTheirDid() returned wrong error code!");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

// MARK: - List pairwise

- (void)testListPairwiseWorks
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. create and open wallet
    
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual( ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed!");
    
    // 2. create and store my did
    
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed1]
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create myDid!");
    
    // 3. create and store their did
    
    NSString *theirDid;
    NSString *theirVerkey;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed2]
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create theirDid!");
    
    // 4. Store their identity json
    
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}", theirDid, theirVerkey];
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual( ret.code, Success, @"DidUtils::storeTheirDidWithWalletHandle() failed!");
    
    // 5. create pairwise
    ret = [[PairwiseUtils sharedInstance] createPairwiseForTheirDid:theirDid
                                                          withMyDid:myDid
                                                           metadata:[TestUtils someMetadata]
                                                       walletHandle:walletHandle];
    XCTAssertEqual( ret.code, Success, @"PairwiseUtils::createPairwiseForTheirDid() failed!");
    
    // 6. list paiwise
    
    NSString *pairwiseListJson;
    ret = [[PairwiseUtils sharedInstance] listPairwiseFromWallet:walletHandle
                                                 outPairwiseList:&pairwiseListJson];
    XCTAssertEqual( ret.code, Success, @"PairwiseUtils::listPairwiseFromWallet() failed!");
    
    XCTAssertTrue([pairwiseListJson isValid], @"pairwiseListJson is invalid: %@", pairwiseListJson);
    
    NSArray *pairwiseList = (NSArray *)[NSDictionary fromString:pairwiseListJson];
    XCTAssertTrue([pairwiseList count] == 1, @"pairwiseList count != 1." );
    
    NSMutableDictionary *pair = [NSMutableDictionary new];
    pair[@"my_did"] = myDid;
    pair[@"their_did"] = theirDid;
    
    XCTAssertTrue([pairwiseList contains:pair], @"pairwiseList doesn't contain pair: %@", pair);
    
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testListPairwiseWorksForEmptyResult
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. create and open wallet
    
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual( ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed!");
    
    // 2. list paiwise
    
    NSString *pairwiseListJson;
    ret = [[PairwiseUtils sharedInstance] listPairwiseFromWallet:walletHandle
                                                 outPairwiseList:&pairwiseListJson];
    XCTAssertEqual( ret.code, Success, @"PairwiseUtils::listPairwiseFromWallet() failed!");
    
    NSArray *pairwiseList = (NSArray *)[NSDictionary fromString:pairwiseListJson];
    XCTAssertTrue([pairwiseList count] == 0, @"pairwiseList count != 0." );
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testIndyListPairwiseWorksForInvalidHandle
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. create and open wallet
    
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual( ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed!");
    
    // 2. create and store my did
    
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed1]
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create myDid!");
    
    // 3. create and store their did
    
    NSString *theirDid;
    NSString *theirVerkey;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed2]
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create theirDid!");
    
    // 4. Store their identity json
    
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}", theirDid, theirVerkey];
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual( ret.code, Success, @"DidUtils::storeTheirDidWithWalletHandle() failed!");
    
    // 5. create pairwise
    ret = [[PairwiseUtils sharedInstance] createPairwiseForTheirDid:theirDid
                                                          withMyDid:myDid
                                                           metadata:[TestUtils someMetadata]
                                                       walletHandle:walletHandle];
    XCTAssertEqual( ret.code, Success, @"PairwiseUtils::createPairwiseForTheirDid() failed!");
    
    // 6. list paiwise
    
    NSString *pairwiseListJson;
    IndyHandle invalidWalletHandle = walletHandle + 1;
    ret = [[PairwiseUtils sharedInstance] listPairwiseFromWallet:invalidWalletHandle
                                                 outPairwiseList:&pairwiseListJson];
    XCTAssertEqual( ret.code, WalletInvalidHandle, @"PairwiseUtils::listPairwiseFromWallet() returned wrong error code!");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

// MARK: - Pairwise exists

- (void)testPairwiseExistsWorks
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. create and open wallet
    
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual( ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed!");
    
    // 2. create and store my did
    
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed1]
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create myDid!");
    
    // 3. create and store their did
    
    NSString *theirDid;
    NSString *theirVerkey;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed2]
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create theirDid!");
    
    // 4. Store their identity json
    
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}", theirDid, theirVerkey];
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual( ret.code, Success, @"DidUtils::storeTheirDidWithWalletHandle() failed!");
    
    // 5. create pairwise
    ret = [[PairwiseUtils sharedInstance] createPairwiseForTheirDid:theirDid
                                                          withMyDid:myDid
                                                           metadata:[TestUtils someMetadata]
                                                       walletHandle:walletHandle];
    XCTAssertEqual( ret.code, Success, @"PairwiseUtils::createPairwiseForTheirDid() failed!");
    
   
    // 6. Check if pairwise exists
    BOOL exists;
    ret = [[PairwiseUtils sharedInstance] pairwiseExistsForDid:theirDid
                                                  walletHandle:walletHandle
                                                     outExists:&exists];
    XCTAssertEqual( ret.code, Success, @"PairwiseUtils::pairwiseExistsForDid() failed!");
    XCTAssertTrue(exists, @"pairwise doesn't exist!");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testIsPairwiseExistsWorksForNotCreated
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. create and open wallet
    
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual( ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed!");
    
    // 2. create and store my did
    
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed1]
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create myDid!");
    
    // 3. create and store their did
    
    NSString *theirDid;
    NSString *theirVerkey;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed2]
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create theirDid!");
    
    // 4. Store their identity json
    
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}", theirDid, theirVerkey];
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual( ret.code, Success, @"DidUtils::storeTheirDidWithWalletHandle() failed!");
    
    // 5. Check if pairwise exists
    BOOL exists = false ;
    ret = [[PairwiseUtils sharedInstance] pairwiseExistsForDid:theirDid
                                                  walletHandle:walletHandle
                                                     outExists:&exists];
    XCTAssertEqual( ret.code, Success, @"PairwiseUtils::pairwiseExistsForDid() failed!");
    XCTAssertFalse(exists, @"pairwise exists!");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testIsPairwiseExistsWorksForInvalidHandle
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. create and open wallet
    
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual( ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed!");
    
    // 2. create and store my did
    
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed1]
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create myDid!");
    
    // 3. create and store their did
    
    NSString *theirDid;
    NSString *theirVerkey;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed2]
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create theirDid!");
    
    // 4. Store their identity json
    
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}", theirDid, theirVerkey];
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual( ret.code, Success, @"DidUtils::storeTheirDidWithWalletHandle() failed!");
    
    // 5. create pairwise
    ret = [[PairwiseUtils sharedInstance] createPairwiseForTheirDid:theirDid
                                                          withMyDid:myDid
                                                           metadata:[TestUtils someMetadata]
                                                       walletHandle:walletHandle];
    XCTAssertEqual( ret.code, Success, @"PairwiseUtils::createPairwiseForTheirDid() failed!");
    
    
    // 6. Check if pairwise exists
    IndyHandle invalidWalletHandle = walletHandle + 1;
    BOOL exists;
    ret = [[PairwiseUtils sharedInstance] pairwiseExistsForDid:theirDid
                                                  walletHandle:invalidWalletHandle
                                                     outExists:&exists];
    XCTAssertEqual( ret.code, WalletInvalidHandle, @"PairwiseUtils::pairwiseExistsForDid() failed!");
    XCTAssertFalse(exists, @"pairwise does exist!");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

// MARK: - Get pairwise

- (void)testGetPairwiseWorks
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. create and open wallet
    
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual( ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed!");
    
    // 2. create and store my did
    
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed1]
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create myDid!");
    
    // 3. create and store their did
    
    NSString *theirDid;
    NSString *theirVerkey;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed2]
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create theirDid!");
    
    // 4. Store their identity json
    
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}", theirDid, theirVerkey];
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual( ret.code, Success, @"DidUtils::storeTheirDidWithWalletHandle() failed!");
    
    // 5. create pairwise
    ret = [[PairwiseUtils sharedInstance] createPairwiseForTheirDid:theirDid
                                                          withMyDid:myDid
                                                           metadata:[TestUtils someMetadata]
                                                       walletHandle:walletHandle];
    XCTAssertEqual( ret.code, Success, @"PairwiseUtils::createPairwiseForTheirDid() failed!");

    // 6. get pairwise
    
    NSString *pairwiseInfoJson;
    ret = [[PairwiseUtils sharedInstance] getPairwiseForDid:theirDid
                                               walletHandle:walletHandle
                                            outPairwiseJson:&pairwiseInfoJson];
    XCTAssertEqual( ret.code, Success, @"PairwiseUtils::getPairwiseForDid() failed!");
    
    XCTAssertTrue([pairwiseInfoJson isValid], @"pairwiseInfoJson is invalid.");
    
    NSDictionary *pairwiseInfo = [NSDictionary fromString:pairwiseInfoJson];
    
    NSMutableDictionary *expectedResult = [NSMutableDictionary new];
    expectedResult[@"my_did"] = myDid;
    expectedResult[@"metadata"] = [TestUtils someMetadata];
    
    XCTAssertTrue([pairwiseInfo contains:expectedResult], @"expectedResult is not contained in pairwiseInfoJson.");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testGetPairwiseWorksForNotCreatedPairwise
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. create and open wallet
    
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual( ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed!");
    
    // 2. create and store my did
    
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed1]
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create myDid!");
    
    // 3. create and store their did
    
    NSString *theirDid;
    NSString *theirVerkey;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed2]
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create theirDid!");
    
    // 4. Store their identity json
    
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}", theirDid, theirVerkey];
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual( ret.code, Success, @"DidUtils::storeTheirDidWithWalletHandle() failed!");
    
    // 5. get pairwise
    
    NSString *pairwiseInfoJson;
    ret = [[PairwiseUtils sharedInstance] getPairwiseForDid:theirDid
                                               walletHandle:walletHandle
                                            outPairwiseJson:&pairwiseInfoJson];
    XCTAssertEqual( ret.code, WalletNotFoundError, @"PairwiseUtils::getPairwiseForDid() returned wrong error code!");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testGetPairwiseWorksForInvalidHandle
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. create and open wallet
    
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual( ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed!");
    
    // 2. create and store my did
    
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed1]
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create myDid!");
    
    // 3. create and store their did
    
    NSString *theirDid;
    NSString *theirVerkey;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed2]
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create theirDid!");
    
    // 4. Store their identity json
    
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}", theirDid, theirVerkey];
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual( ret.code, Success, @"DidUtils::storeTheirDidWithWalletHandle() failed!");
    
    // 5. create pairwise
    ret = [[PairwiseUtils sharedInstance] createPairwiseForTheirDid:theirDid
                                                          withMyDid:myDid
                                                           metadata:[TestUtils someMetadata]
                                                       walletHandle:walletHandle];
    XCTAssertEqual( ret.code, Success, @"PairwiseUtils::createPairwiseForTheirDid() failed!");
    
    // 6. get pairwise
    
    IndyHandle invalidWalletHandle = walletHandle + 1;
    NSString *pairwiseInfoJson;
    ret = [[PairwiseUtils sharedInstance] getPairwiseForDid:theirDid
                                               walletHandle:invalidWalletHandle
                                            outPairwiseJson:&pairwiseInfoJson];
    XCTAssertEqual( ret.code, WalletInvalidHandle, @"PairwiseUtils::getPairwiseForDid() returned wrong error code!");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

// MARK: - Set pairwise metadata

- (void)testSetPairwiseMetadataWorks
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. create and open wallet
    
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual( ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed!");
    
    // 2. create and store my did
    
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed1]
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create myDid!");
    
    // 3. create and store their did
    
    NSString *theirDid;
    NSString *theirVerkey;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed2]
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create theirDid!");
    
    // 4. Store their identity json
    
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}", theirDid, theirVerkey];
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual( ret.code, Success, @"DidUtils::storeTheirDidWithWalletHandle() failed!");
    
    // 5. create pairwise
    ret = [[PairwiseUtils sharedInstance] createPairwiseForTheirDid:theirDid
                                                          withMyDid:myDid
                                                           metadata:nil
                                                       walletHandle:walletHandle];
    XCTAssertEqual( ret.code, Success, @"PairwiseUtils::createPairwiseForTheirDid() failed!");
    
    // 6. get pairwise info without metadata
    
    NSString *pairwiseInfoWithoutMetadataJson;
    ret = [[PairwiseUtils sharedInstance] getPairwiseForDid:theirDid
                                               walletHandle:walletHandle
                                            outPairwiseJson:&pairwiseInfoWithoutMetadataJson];
    XCTAssertEqual( ret.code, Success, @"PairwiseUtils::getPairwiseForDid() failed!");
    
    XCTAssertTrue([pairwiseInfoWithoutMetadataJson isValid], @"pairwiseInfoWithoutMetadataJson is invalid.");
    
    NSDictionary *pairwiseInfoWithoutMetadata = [NSDictionary fromString:pairwiseInfoWithoutMetadataJson];
    
    NSMutableDictionary *expectedResult = [NSMutableDictionary new];
    expectedResult[@"my_did"] = myDid;
    
    XCTAssertTrue([pairwiseInfoWithoutMetadata contains:expectedResult], @"expectedResult is not contained in pairwiseInfoWithoutMetadata.");
    
    // 7. Set pairwise metadata
    
    ret = [[PairwiseUtils sharedInstance] setPairwiseMetadata:[TestUtils someMetadata]
                                                  forTheirDid:theirDid
                                                 walletHandle:walletHandle];
    XCTAssertEqual( ret.code, Success, @"PairwiseUtils::setPairwiseMetadata() failed!");
    
    // 8. get pairwise info with metadata
    
    NSString *pairwiseInfoWithMetadataJson;
    ret = [[PairwiseUtils sharedInstance] getPairwiseForDid:theirDid
                                               walletHandle:walletHandle
                                            outPairwiseJson:&pairwiseInfoWithMetadataJson];
    XCTAssertEqual( ret.code, Success, @"PairwiseUtils::getPairwiseForDid() failed!");
    
    XCTAssertTrue([pairwiseInfoWithMetadataJson isValid], @"pairwiseInfoWithMetadataJson is invalid.");
    
    NSDictionary *pairwiseInfoWithMetadata = [NSDictionary fromString:pairwiseInfoWithMetadataJson];
    
    expectedResult = [NSMutableDictionary new];
    expectedResult[@"my_did"] = myDid;
    expectedResult[@"metadata"] = [TestUtils someMetadata];
    
    XCTAssertTrue([pairwiseInfoWithMetadata contains:expectedResult], @"expectedResult is not contained in pairwiseInfoWithMetadata.");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testSetPairwiseMetadataWorksForNotCreatedPairwise
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. create and open wallet
    
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual( ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed!");
    
    // 2. create and store my did
    
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed1]
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create myDid!");
    
    // 3. create and store their did
    
    NSString *theirDid;
    NSString *theirVerkey;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed2]
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create theirDid!");
    
    // 4. Store their identity json
    
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}", theirDid, theirVerkey];
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual( ret.code, Success, @"DidUtils::storeTheirDidWithWalletHandle() failed!");
    
    // 5. Set pairwise metadata
    
    ret = [[PairwiseUtils sharedInstance] setPairwiseMetadata:[TestUtils someMetadata]
                                                  forTheirDid:theirDid
                                                 walletHandle:walletHandle];
    XCTAssertEqual( ret.code, WalletNotFoundError, @"PairwiseUtils::setPairwiseMetadata() returned wrong error code!");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testSetPairwiseMetadataWorksForInvalidWalletHandle
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. create and open wallet
    
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual( ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed!");
    
    // 2. create and store my did
    
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed1]
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create myDid!");
    
    // 3. create and store their did
    
    NSString *theirDid;
    NSString *theirVerkey;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils mySeed2]
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey];
    XCTAssertEqual( ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed to create theirDid!");
    
    // 4. Store their identity json
    
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}", theirDid, theirVerkey];
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual( ret.code, Success, @"DidUtils::storeTheirDidWithWalletHandle() failed!");
    
    // 5. create pairwise
    ret = [[PairwiseUtils sharedInstance] createPairwiseForTheirDid:theirDid
                                                          withMyDid:myDid
                                                           metadata:nil
                                                       walletHandle:walletHandle];
    XCTAssertEqual( ret.code, Success, @"PairwiseUtils::createPairwiseForTheirDid() failed!");

    
    // 6. Set pairwise metadata
    
    IndyHandle invalidWalletHandle = walletHandle + 1;
    ret = [[PairwiseUtils sharedInstance] setPairwiseMetadata:[TestUtils someMetadata]
                                                  forTheirDid:theirDid
                                                 walletHandle:invalidWalletHandle];
    XCTAssertEqual( ret.code, WalletInvalidHandle, @"PairwiseUtils::setPairwiseMetadata() returned wrong error code!");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

@end
