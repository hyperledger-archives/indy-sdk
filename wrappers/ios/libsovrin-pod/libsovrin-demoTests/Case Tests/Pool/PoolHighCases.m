 //
//  PoolHighCases.m
//  libsovrin-demo
//
//  Created by Anastasia Tarasova on 22.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//


#import <Foundation/Foundation.h>

#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import "WalletUtils.h"
#import <libsovrin/libsovrin.h>
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
    res = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                   nodes:nil
                                                              poolConfig:nil];
    XCTAssertEqual(res.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName failed");
    
    [TestUtils cleanupStorage];
};

- (void)testCreatePoolLedgerConfigWorksForEmptyName
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"";
    NSError *res = nil;
    res = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                   nodes:nil
                                                              poolConfig:nil];
    XCTAssertEqual(res.code, CommonInvalidParam2, @"PoolUtils::createPoolLedgerConfigWithPoolName returned wrong code");
    
    [TestUtils cleanupStorage];
}

// MARK: - Open

- (void) testOpenPoolLedgerWorks
{
    [TestUtils cleanupStorage];
    
    NSString *poolName = @"pool_open";
    
    // 1. Create pool ledger config
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                            nodes:nil
                                                                       poolConfig:nil];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName() failed!");
    
    // 2. Open pool ledger
    SovrinHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName
                                              config:nil
                                         poolHandler:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::openPoolLedger() failed!");
    [TestUtils cleanupStorage];
}

- (void)testOpenPoolLedgerWorksForTwice
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"pool_open_twice";
    
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                            nodes:nil
                                                                       poolConfig:nil];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName() failed!");
    
    SovrinHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName
                                              config:nil
                                         poolHandler:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::openPoolLedger() failed!");
    
    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName
                                              config:nil
                                         poolHandler:&poolHandle];
    // TODO: PoolLedgerInvalidConfiguration is returned.
    XCTAssertEqual(ret.code, PoolLedgerInvalidPoolHandle, @"PoolUtils::openPoolLedger() failed second one!");
    
    [TestUtils cleanupStorage];
}

- (void)testOpenPoolLedgerWorksForTwoNodes
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"open_pool_ledger_works_for_two_nodes";
    
    // 1. Create pool ledger config
    NSString *nodeIp = [PoolUtils nodeIp];
    NSString *nodes = [NSString stringWithFormat:@""
                       "{\"data\":{\"alias\":\"Node1\",\"client_ip\":\"%@\",\"client_port\":9702,\"node_ip\":\"%@\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}\n"
                       "{\"data\":{\"alias\":\"Node2\",\"client_ip\":\"%@\",\"client_port\":9704,\"node_ip\":\"%@\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\",\"identifier\":\"8QhFxKxyaFsJy4CyxeYX34dFH8oWqyBv1P4HLQCsoeLy\",\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"}\n", nodeIp, nodeIp, nodeIp, nodeIp];
    
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                            nodes:nodes
                                                                       poolConfig:nil];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName() failed!");

    // 2. Open pool ledger
    SovrinHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName
                                              config:nil
                                         poolHandler:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::openPoolLedger() failed!");
    [TestUtils cleanupStorage];
}

- (void)testOpenPoolLedgerWorksForThreeNodes
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"open_pool_ledger_works_for_two_nodes";
    
    // 1. Create pool ledger config
    NSString *nodeIp = [PoolUtils nodeIp];
    NSString *nodes = [NSString stringWithFormat:@""
                       "{\"data\":{\"alias\":\"Node1\",\"client_ip\":\"%@\",\"client_port\":9702,\"node_ip\":\"%@\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}\n"
                           "{\"data\":{\"alias\":\"Node2\",\"client_ip\":\"%@\",\"client_port\":9704,\"node_ip\":\"%@\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\",\"identifier\":\"8QhFxKxyaFsJy4CyxeYX34dFH8oWqyBv1P4HLQCsoeLy\",\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"}\n"
                           "{\"data\":{\"alias\":\"Node3\",\"client_ip\":\"%@\",\"client_port\":9706,\"node_ip\":\"%@\",\"node_port\":9705,\"services\":[\"VALIDATOR\"]},\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\",\"identifier\":\"2yAeV5ftuasWNgQwVYzeHeTuM7LwwNtPR3Zg9N4JiDgF\",\"txnId\":\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\",\"type\":\"0\"}\n", nodeIp, nodeIp, nodeIp, nodeIp, nodeIp, nodeIp];
    
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                            nodes:nodes
                                                                       poolConfig:nil];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName() failed!");
    
    // 2. Open pool ledger
    SovrinHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName
                                              config:nil
                                         poolHandler:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::openPoolLedger() failed!");
    [TestUtils cleanupStorage];
}

// MARK - Refresh

- (void)testSovrinRefreshPoolLedgerWorks
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"sovrin_refresh_pool_ledger_works";
    SovrinHandle poolHandle = 0;
    NSError *ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                          poolHandle:&poolHandle];
    
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerConfigWithName() failed!");
    
    ret = [[PoolUtils sharedInstance] refreshPoolHandle:poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::refreshPoolHandle() failed!");
    
     [TestUtils cleanupStorage];
}

// MARK: - Close

- (void)testSovrinClosePoolLedgerWorks
{
    [TestUtils cleanupStorage];
    
    // 1. create and open pool ledger config
    NSString *poolName = @"sovrin_refresh_pool_ledger_works";
    SovrinHandle poolHandle = 0;
    NSError *ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                          poolHandle:&poolHandle];
    
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerConfigWithName() failed!");
    
    // 2. open pool ledger
    ret = [[PoolUtils sharedInstance] closeHandle:poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::closeHandle() failed!");
    
    [TestUtils cleanupStorage];
}

- (void)testSovrinClosePoolLedgerWorksForReopenAfterClose
{
    [TestUtils cleanupStorage];
    
    // 1. create and open pool ledger config
    NSString *poolName = @"sovrin_close_pool_ledger_works";
    SovrinHandle poolHandle = 0;
    NSError *ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                          poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerConfigWithName() failed!");
    
    // 2. close pool ledger
    ret = [[PoolUtils sharedInstance] closeHandle:poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::closeHandle() failed!");
    
    // TODO: 301 error
    // 3. open pool ledger
    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName
                                              config:nil
                                         poolHandler:nil];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::openPoolLedger() returned wrong code!");
    [TestUtils cleanupStorage];
}

// MARK: - Delete

- (void)testSovrinDeletePoolLedgerConfigWorks
{
    [TestUtils cleanupStorage];
    
    // 1. create and open pool ledger config
    NSString *poolName = @"sovrin_close_pool_ledger_works";
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                            nodes:nil
                                                                       poolConfig:nil];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName() failed!");
    
    // 2. delete
    ret = [[PoolUtils sharedInstance] deletePoolWithName:poolName];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::deletePoolWithName() failed!");
    [TestUtils cleanupStorage];
}

- (void)testDeletePoolLedgerConfigWorksForOpened
{
    
    // 1. create and open pool ledger config
    NSString *poolName = @"sovrin_remove_pool_ledger_config_works";
    SovrinHandle poolHandle = 0;
    NSError *ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                          poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerConfigWithName() failed!");
    
    // 2. delete
    // TODO: Returns 0, not 109 error code
    ret = [[PoolUtils sharedInstance] deletePoolWithName:poolName];
    XCTAssertEqual(ret.code, CommonInvalidState, @"PoolUtils::deletePoolWithName() returned wrong code!");
    [TestUtils cleanupStorage];

}

@end
