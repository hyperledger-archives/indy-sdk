//
//  WalletMediumCases.m
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

@interface WalletMediumCases : XCTestCase

@end

@implementation WalletMediumCases

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
- (void)testCreateWalletWorksForDublicateName
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"sovrin_create_wallet_works_for_duplicate_name";
    NSString *walletName = @"wallet1";
    NSError *ret;
    
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName() 1 failed");
    
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:nil];
    XCTAssertEqual(ret.code, WalletAlreadyExistsError, @"WalletUtils:createWalletWithPoolName() returned wrong code");
    [TestUtils cleanupStorage];
}

// MARK: - Delete wallet
- (void)testDeleteWalletWorksForInvalidName
{
    [TestUtils cleanupStorage];
    NSString *walletName = @"wallet1";
    NSError *ret;
    
    ret = [[WalletUtils sharedInstance] deleteWalletWithName:walletName];
    XCTAssertEqual(ret.code, CommonIOError, @"WalletUtils:deleteWalletWithName() returned wrong error");

    [TestUtils cleanupStorage];
}

- (void)testDeleteWalletWorksForDeletedWallet
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"sovrin_delete_wallet_works_for_deleted_wallet";
    NSString *walletName = @"wallet1";
    NSError *ret;
    
    // 1. create wallet
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName() failed");
    
    // 2. delete wallet
    ret = [[WalletUtils sharedInstance] deleteWalletWithName:walletName];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:deleteWalletWithName() failed");
    
    // 3. delete walled again
    ret = [[WalletUtils sharedInstance] deleteWalletWithName:walletName];
    XCTAssertEqual(ret.code, CommonIOError, @"WalletUtils:deleteWalletWithName() returned wrong code");
    
    [TestUtils cleanupStorage];
}

// MARK: - Open wallet
- (void)testOpenWalletWorksForNotCreatedWallet
{
    [TestUtils cleanupStorage];
    NSString *walletName = @"wallet1";
    NSError *ret;
    
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName
                                                 outHandle:nil];
    XCTAssertEqual(ret.code, CommonIOError, @"WalletUtils:openWalletWithName() failed");
    
    [TestUtils cleanupStorage];
}

// MARK: - Close wallet

- (void)testCloseWalletWorksForInvalidHandle
{
    [TestUtils cleanupStorage];
    SovrinHandle walletHandle = 1;
    
    NSError *ret = [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"WalletUtils:closeWalletWithHandle() returned wrong code");
    
    [TestUtils cleanupStorage];
}

- (void)testCloseWalletWorksForClosedWallet
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"sovrin_close_wallet_works_for_closed_wallet";
    NSString *walletName = @"wallet1";
    NSString *xtype = @"default";
    NSError *ret;
    
    // 1. create and open wallet
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName() failed");
    
    // 2. close wallet
    ret = [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:closeWalletWithHandle() failed");
    
    // 3. close wallet
    ret = [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"WalletUtils:deleteWalletWithName() returned wrong code");
    
    [TestUtils cleanupStorage];
}

// MARK: - Set seqNo

- (void)testWalletSetSeqNoWorksForNotExistsKey
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"sovrin_wallet_set_seqno_works_for_not_exists_key";
    NSString *walletName = @"wallet1";
    NSString *xtype = @"default";
    NSError *ret;
    
    // 1. create and open wallet
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName() failed");
    
    // 2. set seqNo
    NSNumber *seqNo = @(1);
    NSString *someKey = @"key";
    ////TODO may be we must return WalletNotFound in case if key not exists in wallet
    ret = [[WalletUtils sharedInstance] walletSetSeqNoForValue:walletHandle
                                                  claimDefUUID:someKey
                                                 claimDefSeqNo:seqNo];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:walletSetSeqNoForValue() failed");
    [TestUtils cleanupStorage];
}

- (void)testWalletSetSeqNoWorksForInvalidWallet
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"sovrin_wallet_set_seqno_works_for_not_exists_key";
    NSString *walletName = @"wallet1";
    NSError *ret;
    
    // 1. create and open wallet
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:@"default"
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName() failed");
    
    // 2. set seqNo
    NSNumber *seqNo = @(1);
    NSString *someKey = @"key";
    SovrinHandle invalidWalletHandle = walletHandle + 1;
    ////TODO may be we must return WalletNotFound in case if key not exists in wallet
    ret = [[WalletUtils sharedInstance] walletSetSeqNoForValue:invalidWalletHandle
                                                  claimDefUUID:someKey
                                                 claimDefSeqNo:seqNo];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"WalletUtils:walletSetSeqNoForValue() failed");
    [TestUtils cleanupStorage];
}

@end
