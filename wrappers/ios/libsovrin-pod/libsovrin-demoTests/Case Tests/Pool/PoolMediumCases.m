//
//  PoolMediumCases.m
//  libsovrin-demo
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
#import <libsovrin/libsovrin.h>
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
                                                                            nodes:nil
                                                                       poolConfig:config
                                                                   genTxnFileName:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"PoolUtils::createPoolLedgerConfigWithPoolName returned wrong error code");
    
    [TestUtils cleanupStorage];
}

- (void)testPoolLedgerConfigWorksForInvalidGenesisTxnPath
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"create_pool_ledger_config_works_for_invalid_genesis_txn_path";
    NSString *config = @"{\"genesis_txn\": \"path\"}";
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                            nodes:nil
                                                                       poolConfig:config
                                                                   genTxnFileName:nil];
    XCTAssertEqual(ret.code, CommonIOError, @"PoolUtils::createPoolLedgerConfigWithPoolName returned wrong error code");
    [TestUtils cleanupStorage];
}

- (void)testCreatePoolLedgerConfigWorksForTwice
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"pool_create";
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                            nodes:nil
                                                                       poolConfig:nil
                                                                   genTxnFileName:nil];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName failed");
    
    ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                   nodes:nil
                                                              poolConfig:nil
                                                          genTxnFileName:nil];
    XCTAssertEqual(ret.code, PoolLedgerNotCreatedError, @"PoolUtils::createPoolLedgerConfigWithPoolName returned wrong code");
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
    ////TODO change it on IOError
    XCTAssertEqual(ret.code, PoolLedgerTerminated, @"PoolUtils::openPoolLedger returned wrong code");
    [TestUtils cleanupStorage];
}

- (void)testOpenPoolLedgerWorksForInvalidNodesFile
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"open_pool_ledger_works_for_invalid_nodes_file";
    NSString *nodeIp = [PoolUtils nodeIp];
    NSString *nodes = [NSString stringWithFormat:@""
                       "{\"data\":{\"client_port\":9702,\"client_ip\":\"%@\",\"node_ip\":\"%@\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}\n"
        "{\"data\":{\"client_port\":9704,\"client_ip\":\"%@\",\"node_ip\":\"%@\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\",\"identifier\":\"8QhFxKxyaFsJy4CyxeYX34dFH8oWqyBv1P4HLQCsoeLy\",\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"}\n"
        "{\"data\":{\"client_port\":9706,\"client_ip\":\"%@\",\"node_ip\":\"%@\",\"node_port\":9705,\"services\":[\"VALIDATOR\"]},\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\",\"identifier\":\"2yAeV5ftuasWNgQwVYzeHeTuM7LwwNtPR3Zg9N4JiDgF\",\"txnId\":\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\",\"type\":\"0\"}\n"
        "{\"data\":{\"client_port\":9708,\"client_ip\":\"%@\",\"node_ip\":\"%@\",\"node_port\":9707,\"services\":[\"VALIDATOR\"]},\"dest\":\"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\",\"identifier\":\"FTE95CVthRtrBnK2PYCBbC9LghTcGwi9Zfi1Gz2dnyNx\",\"txnId\":\"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008\",\"type\":\"0\"}\n",nodeIp, nodeIp, nodeIp, nodeIp, nodeIp, nodeIp, nodeIp, nodeIp];
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                            nodes:nodes
                                                                       poolConfig:nil
                                                                   genTxnFileName:nil];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName failed");
    
    // 2. open pool ledger
    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName
                                              config:nil
                                         poolHandler:nil];
    XCTAssertEqual(ret.code, PoolLedgerTerminated, @"PoolUtils::openPoolLedger returned wrong code");
    [TestUtils cleanupStorage];
}

// TODO: Some bug in Rust
- (void)testOpenPoolLedgerWorksForWrongAlias
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"open_pool_ledger_works_for_wrong_alias";
    NSString *nodeIp = [PoolUtils nodeIp];
    NSString *node1 = [NSString stringWithFormat:@"{\"data\":{\"alias\":\"Node1\",\"client_ip\":\"%@\",\"client_port\":9702,\"node_ip\":\"%@\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}", nodeIp, nodeIp];
    NSString *node2 = [NSString stringWithFormat:@"{\"data\":{\"alias\":\"Node2\",\"client_ip\":\"%@\",\"client_port\":9704,\"node_ip\":\"%@\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\",\"identifier\":\"8QhFxKxyaFsJy4CyxeYX34dFH8oWqyBv1P4HLQCsoeLy\",\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"}",nodeIp, nodeIp];
    NSString *node3 = [NSString stringWithFormat:@ "{\"data\":{\"alias\":\"Node3\",\"client_ip\":\"%@\",\"client_port\":9706,\"node_ip\":\"%@\",\"node_port\":9705,\"services\":[\"VALIDATOR\"]},\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\",\"identifier\":\"2yAeV5ftuasWNgQwVYzeHeTuM7LwwNtPR3Zg9N4JiDgF\",\"txnId\":\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\",\"type\":\"0\"}", nodeIp, nodeIp];
    NSString *node4 = [NSString stringWithFormat:@"{\"data\":{\"alias\":\"ALIAS_NODE\",\"client_ip\":\"%@\",\"client_port\":9708,\"node_ip\":\"%@\",\"node_port\":9707,\"services\":[\"VALIDATOR\"]},\"dest\":\"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\",\"identifier\":\"FTE95CVthRtrBnK2PYCBbC9LghTcGwi9Zfi1Gz2dnyNx\",\"txnId\":\"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008\",\"type\":\"0\"}", nodeIp, nodeIp];
    NSString *nodes = [NSString stringWithFormat:@"%@\n%@\n%@\n%@\n",
                       node1, node2, node3, node4];
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                            nodes:nodes
                                                                       poolConfig:nil
                                                                   genTxnFileName:nil];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName failed");
    
    // 2. open pool ledger
    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName
                                              config:nil
                                         poolHandler:nil];
    // TODO: returns 0, not 302
    XCTAssertEqual(ret.code, PoolLedgerTerminated, @"PoolUtils::openPoolLedger returned wrong code");
    [TestUtils cleanupStorage];
}

// TODO: This test is ignored in rust, not implemeted yet
//- (void)testOpenPoolLedgerWorksForInvalidConfig
//{
//    [TestUtils cleanupStorage];
//    NSString *poolName = @"pool_open";
//    NSString *config = @"{\"refreshOnOpen\": \"true\"}";
//    
//    // 1. create pool ledger
//    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
//                                                                            nodes:nil
//                                                                       poolConfig:config];
//    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName failed");
//    
//    // 2. open pool ledger
//    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName
//                                              config:nil
//                                         poolHandler:nil];
//    XCTAssertEqual(ret.code, CommonInvalidStructure, @"PoolUtils::openPoolLedger returned wrong code");
//    
//    [TestUtils cleanupStorage];
//}

@end
