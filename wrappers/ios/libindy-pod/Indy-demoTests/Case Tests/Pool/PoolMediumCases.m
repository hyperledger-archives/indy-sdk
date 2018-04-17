//
//  PoolMediumCases.m
//  Indy-demo
//
//  Created by Anastasia Tarasova on 22.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>

#import <Foundation/Foundation.h>

#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import "WalletUtils.h"
#import <Indy/Indy.h>
#import "NSDictionary+JSON.h"

@interface PoolMediumCases : XCTestCase

@end

@implementation PoolMediumCases

// MARK: - Create

- (void)testCreatePoolLedgerConfigWorksForInvalidConfigJson
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"create_pool_ledger_config_works_for_invalid_config";
    NSString *config = @"{}";
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                       poolConfig:config];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"PoolUtils::createPoolLedgerConfigWithPoolName returned wrong error code");
    
    [TestUtils cleanupStorage];
}

- (void)testPoolLedgerConfigWorksForInvalidGenesisTxnPath
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"create_pool_ledger_config_works_for_invalid_genesis_txn_path";
    NSString *config = @"{\"genesis_txn\": \"path\"}";
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                       poolConfig:config];
    XCTAssertEqual(ret.code, CommonIOError, @"PoolUtils::createPoolLedgerConfigWithPoolName returned wrong error code");
    [TestUtils cleanupStorage];
}

- (void)testCreatePoolLedgerConfigWorksForTwice
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"pool_create";
    
    NSString *txnFilePath = [[PoolUtils sharedInstance] createGenesisTxnFileForTestPool:poolName
                                                                             nodesCount:nil
                                                                            txnFilePath:nil];
    NSString *poolConfig = [[PoolUtils sharedInstance] poolConfigJsonForTxnFilePath:txnFilePath];
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                       poolConfig:poolConfig];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName failed");
    
    ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                              poolConfig:poolConfig];
    XCTAssertEqual(ret.code, PoolLedgerConfigAlreadyExistsError, @"PoolUtils::createPoolLedgerConfigWithPoolName returned wrong code");
    [TestUtils cleanupStorage];
}

// MARK: - Open

- (void)testOpenPoolLedgerWorksForInvalidName
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"open_pool_ledger_works_for_invalid_name";
    NSError *ret = [[PoolUtils sharedInstance] openPoolLedger:poolName
                                                       config:nil
                                                  poolHandler:nil];
    XCTAssertEqual(ret.code, PoolLedgerNotCreatedError, @"PoolUtils::openPoolLedger returned wrong code");
    [TestUtils cleanupStorage];
}

//TODO ignored in Rust
- (void)testOpenPoolLedgerWorksForInvalidNodesFile
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"open_pool_ledger_works_for_invalid_nodes_file";
    
    NSString *txnFilePath = [[PoolUtils sharedInstance] createGenesisTxnFileForTestPoolWithInvalidNodesForPoolName:poolName
                                                                                                       txnFilePath:nil];
    NSString *poolConfig = [[PoolUtils sharedInstance] poolConfigJsonForTxnFilePath:txnFilePath];
    
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                       poolConfig:poolConfig];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName failed");
    
    // 2. open pool ledger
    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName
                                              config:nil
                                         poolHandler:nil];
    XCTAssertEqual(ret.code, CommonInvalidState, @"PoolUtils::openPoolLedger returned wrong code");
    [TestUtils cleanupStorage];
}

// TODO: Some bug in Rust
- (void)testOpenPoolLedgerWorksForWrongAlias
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"open_pool_ledger_works_for_wrong_alias";
  
    NSString *txnFilePath = [[PoolUtils sharedInstance] createGenesisTxnFileForTestPoolWithWrongAliasForPoolName:poolName
                                                                                                     txnFilePath:txnFilePath];
    NSString *poolConfig = [[PoolUtils sharedInstance] poolConfigJsonForTxnFilePath:txnFilePath];
    
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                       poolConfig:poolConfig];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName failed");

    
    // 2. open pool ledger
    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName
                                              config:nil
                                         poolHandler:nil];

    XCTAssertEqual(ret.code, CommonInvalidState, @"PoolUtils::openPoolLedger returned wrong code");
    [TestUtils cleanupStorage];
}

 //TODO: - not implemented yet in Rust
- (void)testOpenPoolLedgerWorksForInvalidConfig
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"pool_open";
    NSString *config = @"{\"refresh_on_open\": \"true\"}";
    
    // 1. create pool ledger
    NSString *txnFilePath = [[PoolUtils sharedInstance] createGenesisTxnFileForTestPool:poolName
                                                                             nodesCount:nil
                                                                            txnFilePath:nil];
    NSString *poolConfig = [[PoolUtils sharedInstance] poolConfigJsonForTxnFilePath:txnFilePath];
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                       poolConfig:poolConfig];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName failed");
    
    // 2. open pool ledger
    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName
                                              config:config
                                         poolHandler:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"PoolUtils::openPoolLedger returned wrong code");
    
    [TestUtils cleanupStorage];
}

// MARK: - Close

- (void)testClosePoolLedgerWorksForInvalidHandle
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_close_pool_ledger_works_for_invalid_handle";
    
    // 1. create and open pool ledger
    
    IndyHandle poolHandle = 0;
    NSError *ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                                        poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName failed");
    
    // 1. Close
    
    IndyHandle invalidPoolHandle = poolHandle + 1;
    ret = [[PoolUtils sharedInstance] closeHandle:invalidPoolHandle];
    XCTAssertEqual(ret.code, PoolLedgerInvalidPoolHandle, @"PoolUtils::closeHandle returned wrong code");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

// MARK: - Delete

- (void)testDeletePoolLedgerConfigWorksForNotCreated
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_delete_pool_ledger_config_works_for_invalid_name";
    
    // 1. delete
    NSError *ret = [[PoolUtils sharedInstance] deletePoolWithName:poolName];
    XCTAssertEqual(ret.code, CommonIOError, @"PoolUtils::deletePoolWithName returned wrong code");
    
    [TestUtils cleanupStorage];
}

- (void)testDeletePoolLedgerConfigWorksForTwice
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_delete_pool_ledger_config_works_for_twice";
    
    // 1. create and open pool ledger
    
    IndyHandle poolHandle = 0;
    NSError *ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                                        poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName failed");
    
    // 2. close
    [[PoolUtils sharedInstance] closeHandle: poolHandle];
    
    // 3. delete
    [[PoolUtils sharedInstance] deletePoolWithName:poolName];
    
    // 4. delete
    ret = [[PoolUtils sharedInstance] deletePoolWithName:poolName];
    XCTAssertEqual(ret.code, CommonIOError, @"PoolUtils::deletePoolWithName returned wrong code");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

// MARK: - Refresh

- (void)testRefreshPoolLedgerWorksForInvalidHandle
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_refresh_pool_ledger_works_for_invalid_handle";
    
    // 1. create and open pool ledger
    
    IndyHandle poolHandle = 0;
    NSError *ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                                        poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName failed");
    
    // 1. refresh
    
    IndyHandle invalidPoolHandle = poolHandle + 1;
    ret = [[PoolUtils sharedInstance] refreshPoolHandle:invalidPoolHandle];
    XCTAssertEqual(ret.code, PoolLedgerInvalidPoolHandle, @"PoolUtils::refreshPoolHandle returned wrong code");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

@end
