 //
//  PoolHighCases.m
//  libindy-demo
//
//  Created by Anastasia Tarasova on 22.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//


#import <Foundation/Foundation.h>

#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import "WalletUtils.h"
#import <libindy/libindy.h>
#import "NSDictionary+JSON.h"

@interface PoolHighCases : XCTestCase

@end

@implementation PoolHighCases

// MARK: - Create
- (void) testCreatePoolLedgerConfigWorks
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"pool_create";
    NSError *res = nil;
    
    NSString *txnFilePath = [[PoolUtils sharedInstance] createGenesisTxnFileForTestPool:poolName
                                                                             nodesCount:nil
                                                                            txnFilePath:nil];
    NSString *poolConfig = [[PoolUtils sharedInstance] poolConfigJsonForTxnFilePath:txnFilePath];
    
    res = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                              poolConfig:poolConfig];
    XCTAssertEqual(res.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName failed");
    
    [TestUtils cleanupStorage];
};

- (void)testCreatePoolLedgerConfigWorksForEmptyName
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"";
    NSError *res = nil;
    res = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                              poolConfig:nil];
    XCTAssertEqual(res.code, CommonInvalidParam2, @"PoolUtils::createPoolLedgerConfigWithPoolName returned wrong code");
    [TestUtils cleanupStorage];
}

- (void)testCreatePoolLedgerConfigWorksForConfigJson
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"create_pool_ledger_config_works_for_config_json";
    
    NSString *txnFilePath = [[PoolUtils sharedInstance] createGenesisTxnFileForTestPool:poolName
                                                                             nodesCount:nil
                                                                            txnFilePath:nil];
    NSString *poolConfig = [[PoolUtils sharedInstance] poolConfigJsonForTxnFilePath:txnFilePath];

    
    NSError * res = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                        poolConfig:poolConfig];
    XCTAssertEqual(res.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName returned wrong code");
    [TestUtils cleanupStorage];
}

- (void)testCreatePoolLedgerConfigWorksForSpecificConfig
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"create_pool_ledger_config_works_for_specific_config";
    NSString *txnFilePath = [TestUtils tmpFilePathAppending:@"specific_filename.txn"];
    txnFilePath = [[PoolUtils sharedInstance] createGenesisTxnFileForTestPool:poolName
                                                                             nodesCount:nil
                                                                            txnFilePath:txnFilePath];
    NSString *poolConfig = [[PoolUtils sharedInstance] poolConfigJsonForTxnFilePath:txnFilePath];
    
    NSError * res = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                        poolConfig:poolConfig];
    XCTAssertEqual(res.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName returned wrong code");
    [TestUtils cleanupStorage];
}

// MARK: - Open

- (void) testOpenPoolLedgerWorks
{
    [TestUtils cleanupStorage];
    
    NSString *poolName = @"pool_open";
    
    NSString *txnFilePath = [[PoolUtils sharedInstance] createGenesisTxnFileForTestPool:poolName
                                                                             nodesCount:nil
                                                                            txnFilePath:nil];
    NSString *poolConfig = [[PoolUtils sharedInstance] poolConfigJsonForTxnFilePath:txnFilePath];
    
    // 1. Create pool ledger config
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                       poolConfig:poolConfig];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName() failed!");
    
    // 2. Open pool ledger
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName
                                              config:nil
                                         poolHandler:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::openPoolLedger() failed!");
    [TestUtils cleanupStorage];
}

- (void)testOpenPoolLedgerWorksForConfig
{
    [TestUtils cleanupStorage];
    
    NSString *poolName = @"open_pool_ledger_works_for_config";
    NSString *config = @"{\"refresh_on_open\": true}";
    
    // 1. Create pool ledger config
    
    NSString *txnFilePath = [[PoolUtils sharedInstance] createGenesisTxnFileForTestPool:poolName
                                                                             nodesCount:nil
                                                                            txnFilePath:nil];
    NSString *poolConfig = [[PoolUtils sharedInstance] poolConfigJsonForTxnFilePath:txnFilePath];

    
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                       poolConfig:poolConfig];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName() failed!");
    
    // 2. Open pool ledger
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName
                                              config:config
                                         poolHandler:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::openPoolLedger() failed!");
    [TestUtils cleanupStorage];
    
}

- (void)testOpenPoolLedgerWorksForTwice
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"pool_open_twice";
    
    // 1. create pool ledger config
    NSError *ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                                        poolHandle:nil];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName() failed!");
    
    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName
                                              config:nil
                                         poolHandler:nil];
    XCTAssertEqual(ret.code, PoolLedgerInvalidPoolHandle, @"PoolUtils::openPoolLedger() failed second one!");
    
    [TestUtils cleanupStorage];
}

- (void)testOpenPoolLedgerWorksForTwoNodes
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"open_pool_ledger_works_for_two_nodes";
    
    NSString *txnFilePath = [[PoolUtils sharedInstance] createGenesisTxnFileForTestPool:poolName
                                                                             nodesCount:[NSNumber numberWithInt:2]
                                                                            txnFilePath:nil];
    NSString *poolConfig = [[PoolUtils sharedInstance] poolConfigJsonForTxnFilePath:txnFilePath];
    
    // 1. Create pool ledger config
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                       poolConfig:poolConfig];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName() failed!");

    // 2. Open pool ledger
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName
                                              config:nil
                                         poolHandler:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::openPoolLedger() failed!");
    [TestUtils cleanupStorage];
}

- (void)testOpenPoolLedgerWorksForThreeNodes
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"open_pool_ledger_works_for_three_nodes";
    
    NSString *txnFilePath = [[PoolUtils sharedInstance] createGenesisTxnFileForTestPool:poolName
                                                                             nodesCount:[NSNumber numberWithInt:3]
                                                                            txnFilePath:nil];
    NSString *poolConfig = [[PoolUtils sharedInstance] poolConfigJsonForTxnFilePath:txnFilePath];
    
    // 1. Create pool ledger config
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                       poolConfig:poolConfig];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName() failed!");

    
    // 2. Open pool ledger
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName
                                              config:nil
                                         poolHandler:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::openPoolLedger() failed!");
    [TestUtils cleanupStorage];
}

// MARK - Refresh

- (void)testIndyRefreshPoolLedgerWorks
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_refresh_pool_ledger_works";
    
    IndyHandle poolHandle = 0;
    NSError *ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                                        poolHandle:&poolHandle];
    
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed!");
    
    ret = [[PoolUtils sharedInstance] refreshPoolHandle:poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::refreshPoolHandle() failed!");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

// MARK: - Close

- (void)testClosePoolLedgerWorks
{
    [TestUtils cleanupStorage];
    
    // 1. create and open pool ledger config
    NSString *poolName = @"indy_refresh_pool_ledger_works";
    
    IndyHandle poolHandle = 0;
    NSError *ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                                        poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed!");
    
    // 2. close pool ledger
    ret = [[PoolUtils sharedInstance] closeHandle:poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::closeHandle() failed!");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

- (void)testClosePoolLedgerWorksForTwice
{
    [TestUtils cleanupStorage];
    
    // 1. create and open pool ledger config
    NSString *poolName = @"indy_refresh_pool_ledger_works";
    IndyHandle poolHandle = 0;
    NSError *ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                                        poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed!");
    
    // 2. close pool ledger
    ret = [[PoolUtils sharedInstance] closeHandle:poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::closeHandle() failed!");
    
    // 3. close pool ledger
    ret = [[PoolUtils sharedInstance] closeHandle:poolHandle];
    XCTAssertEqual(ret.code, PoolLedgerInvalidPoolHandle, @"PoolUtils::closeHandle() returned wrong code!");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

- (void)testClosePoolLedgerWorksForReopenAfterClose
{
    [TestUtils cleanupStorage];
    
    // 1. create and open pool ledger config
    NSString *poolName = @"indy_close_pool_ledger_works";
    IndyHandle poolHandle = 0;
    NSError *ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                                        poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed!");
    
    // 2. close pool ledger
    ret = [[PoolUtils sharedInstance] closeHandle:poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::closeHandle() failed!");
    
    // 3. open pool ledger
    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName
                                              config:nil
                                         poolHandler:nil];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::openPoolLedger() returned wrong code!");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

// MARK: - Delete

- (void)testIndyDeletePoolLedgerConfigWorks
{
    [TestUtils cleanupStorage];
    
    // 1. create and open pool ledger config
    NSString *poolName = @"indy_remove_pool_ledger_config_works";
    
    NSString *txnFilePath = [[PoolUtils sharedInstance] createGenesisTxnFileForTestPool:poolName
                                                                             nodesCount:nil
                                                                            txnFilePath:nil];
    NSString *poolConfig = [[PoolUtils sharedInstance] poolConfigJsonForTxnFilePath:txnFilePath];
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                       poolConfig:poolConfig];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName() failed!");
    
    // 2. delete
    ret = [[PoolUtils sharedInstance] deletePoolWithName:poolName];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::deletePoolWithName() failed!");
    
    [TestUtils cleanupStorage];
}

- (void)testDeletePoolLedgerConfigWorksForOpened
{
    
    // 1. create and open pool ledger config
    NSString *poolName = @"indy_remove_pool_ledger_config_works_for_opened";
    IndyHandle poolHandle = 0;
    NSError *ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                                        poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed!");
    
    // 2. delete
    ret = [[PoolUtils sharedInstance] deletePoolWithName:poolName];
    XCTAssertEqual(ret.code, CommonInvalidState, @"PoolUtils::deletePoolWithName() returned wrong code!");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

@end
