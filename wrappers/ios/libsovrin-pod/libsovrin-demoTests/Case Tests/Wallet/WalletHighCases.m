//
//  WalletHighCases.m
//  libsovrin-demo
//
//  Created by Anastasia Tarasova on 14.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//


#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import <libsovrin/libsovrin.h>
#import "WalletUtils.h"
#import "SignusUtils.h"
#import "LedgerUtils.h"
#import "AnoncredsUtils.h"
#import "NSDictionary+JSON.h"

@interface WalletHignCases : XCTestCase

@end

@implementation WalletHignCases

- (void)setUp
{
    [super setUp];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown
{
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

// MARK: - Create wallet

- (void)testCreateWalletWorks
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"sovrin_create_wallet_works";
    
    NSError *ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                               walletName:@"wallet1"
                                                                    xtype:@"default"
                                                                   config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName() failed");
    
    [TestUtils cleanupStorage];
}

- (void)testCreateWalletWorksForUnknownType
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"sovrin_create_wallet_works_for_unknown_type";
    
    NSError *ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                               walletName:@"wallet1"
                                                                    xtype:@"type"
                                                                   config:nil];
    XCTAssertEqual(ret.code, WalletUnknownTypeError, @"WalletUtils:createWalletWithPoolName() returned wrong error");
    
    [TestUtils cleanupStorage];
}

- (void)testCreateWalletWorksForEmptyType
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"sovrin_create_wallet_works_for_empty_type";
    
    NSError *ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                               walletName:@"wallet1"
                                                                    xtype:nil
                                                                   config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");
    
    [TestUtils cleanupStorage];
}

- (void)testCreateWalletWorksForConfig
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"sovrin_create_wallet_works";
    NSString *config = @"{\"freshness_time\":1000}";
    
    NSError *ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                               walletName:@"wallet1"
                                                                    xtype:@"default"
                                                                   config:config];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");
    
    [TestUtils cleanupStorage];
}

// MARK: - Delete wallet

- (void)testDeleteWalletWorks
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"sovrin_delete_wallet_works";
    NSString *walletName = @"wallet1";
    NSError *ret;
    
    // 1. Create wallet
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:nil
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");
    
    // 2. Delete wallet
    ret = [[WalletUtils sharedInstance] deleteWalletWithName:walletName];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:deleteWalletWithName failed");
    
    // 3. Create wallet
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:nil
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");
    
    [TestUtils cleanupStorage];
}

// MARK: - Open wallet
- (void)testOpenWalletWorks
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"sovrin_open_wallet_works";
    NSString *walletName = @"wallet1";
    NSError *ret;
    
    // 1. Create wallet
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:nil
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");
    
    // 2. Open wallet
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName
                                                    config:nil
                                                 outHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName failed");
    
    [TestUtils cleanupStorage];
}

- (void)testOpenWalletWorksForConfig
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"sovrin_open_wallet_works";
    NSString *config = @"{\"freshness_time\":1000}";
    NSString *walletName = @"wallet1";
    NSError *ret;
    
    // 1. Create wallet
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:@"default"
                                                          config:config];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");
    
    // 2. Open wallet
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName
                                                    config:config
                                                 outHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName failed");
    
    [TestUtils cleanupStorage];
}

// MARK: - Close wallet
- (void)testCloseWalletWorks
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"sovrin_close_wallet_works";
    NSString *walletName = @"wallet1";
    NSError *ret;
    
    // 1. create wallet
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:nil
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");
    
    // 2. open wallet
    SovrinHandle walletHandle;
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName
                                                    config:nil
                                                 outHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName failed");
    
    // 3. close wallet
    ret = [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:closeWalletWithHandle failed");
    
    // 4. open wallet
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName
                                                    config:nil
                                                 outHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName failed");
    
    [TestUtils cleanupStorage];
}

// MARK: - Set seq no
- (void)testSetSeqnoWallet
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"sovrin_wallet_set_seqno_works";
    NSString *walletName = @"wallet1";
    NSError *ret;
    
    // 1. create and open wallet
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:@"default"
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName() failed");
    
    // 2. Get claimDefUUID
    NSNumber *schemaSeqNo = @(1);
    NSNumber *claimDefSeqNo = @(1);
    
    NSString *schema = [[AnoncredsUtils sharedInstance] getGvtSchemaJson:schemaSeqNo];
    NSString *claimDefUUID;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:walletHandle
                                                                            schemaJson:schema
                                                                        createNonRevoc:NO
                                                                          claimDefJson:nil
                                                                          claimDefUUID:&claimDefUUID];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils:issuerCreateClaimDefinifionWithWalletHandle() failed");
    
    // 3. Wallet set seq no for no value
    ret = [[WalletUtils sharedInstance] walletSetSeqNoForValue:walletHandle
                                                  claimDefUUID:claimDefUUID
                                                 claimDefSeqNo:claimDefSeqNo];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:walletSetSeqNoForValue() failed");
    
    [TestUtils cleanupStorage];
}


@end
