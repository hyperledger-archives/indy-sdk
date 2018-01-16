//
//  WalletMediumCases.m
//  Indy-demo
//
//  Created by Anastasia Tarasova on 14.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//


#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import <Indy/Indy.h>
#import "WalletUtils.h"
#import "DidUtils.h"
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

// MARK: - Register wallet

- (void)testRegisterWalletTypeDoesNotWorkTwiceWithTheSameName
{
    [TestUtils cleanupStorage];
   [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
    
    NSError *ret;
    NSString *xtype = @"keychain";
    
    // 1. register
    ret = [[WalletUtils sharedInstance] registerWalletType:xtype];
    
    // 2. register
    ret = [[WalletUtils sharedInstance] registerWalletType:xtype];
    XCTAssertEqual(ret.code, WalletTypeAlreadyRegisteredError, @"WalletUtils:registerWalletType() returned wrong error code");
    
   [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
    [TestUtils cleanupStorage];
}


// MARK: - Create wallet
- (void)testCreateWalletWorksForDublicateName
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_create_wallet_works_for_duplicate_name";
    NSString *walletName = @"indy_create_wallet_works_for_duplicate_name";
    NSError *ret;
    
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:nil
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName() failed");
    
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:nil
                                                          config:nil];
    XCTAssertEqual(ret.code, WalletAlreadyExistsError, @"WalletUtils:createWalletWithPoolName() returned wrong code");
    [TestUtils cleanupStorage];
}

- (void)testCreateWalletWorksForEmptyName
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_create_wallet_works_for_empty_name";
    NSString *walletName = @"";
    
    NSError *ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                               walletName:walletName
                                                                    xtype:nil
                                                                   config:nil];
     XCTAssertEqual(ret.code, CommonInvalidParam3, @"WalletUtils:createWalletWithPoolName() returned wrong code");
    
    [TestUtils cleanupStorage];
}

// MARK: - Delete wallet
- (void)testDeleteWalletWorksForInvalidName
{
    [TestUtils cleanupStorage];
    NSString *walletName = @"indy_delete_wallet_works_for_invalid_wallet_name";
    NSError *ret;
    
    ret = [[WalletUtils sharedInstance] deleteWalletWithName:walletName];
    XCTAssertEqual(ret.code, CommonIOError, @"WalletUtils:deleteWalletWithName() returned wrong error");
    
    [TestUtils cleanupStorage];
}

- (void)testDeleteWalletWorksForTwice
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_delete_wallet_works_for_deleted_wallet";
    NSString *walletName = @"indy_delete_wallet_works_for_deleted_wallet";
    NSError *ret;
    
    // 1. create wallet
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:nil
                                                          config:nil];
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
    NSString *walletName = @"indy_open_wallet_works_for_not_created_wallet";
    NSError *ret;
    
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName
                                                    config:nil
                                                 outHandle:nil];
    XCTAssertEqual(ret.code, CommonIOError, @"WalletUtils:openWalletWithName() failed");
    
    [TestUtils cleanupStorage];
}

- (void)testOpenWalletWorksForTwice
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_create_wallet_works";
    NSString *walletName = @"indy_open_wallet_works_for_twice";
    NSError *ret;
    
    // 1. Create wallet
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:nil
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName() failed");
    
    // 2. Open wallet
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName
                                                    config:nil
                                                 outHandle:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName() failed");
    
    
    // 3. Open wallet again
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName
                                                    config:nil
                                                 outHandle:nil];
    XCTAssertEqual(ret.code, WalletAlreadyOpenedError, @"WalletUtils:openWalletWithName() failed");
    
    [TestUtils cleanupStorage];
}

- (void)testOpenWalletWorksForTwoWallets
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_create_wallet_works";
    NSString *walletName1 = @"indy_open_wallet_works_for_two_wallets1";
    NSString *walletName2 = @"indy_open_wallet_works_for_two_wallets2";
    NSError *ret;
    
    // 1. Create wallet 1
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName1
                                                           xtype:nil
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName() failed for wallet 1");
    
    // 1. Create wallet 2
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName2
                                                           xtype:nil
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName() failed for wallet 2");
    
    // 2. Open wallet 1
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName1
                                                    config:nil
                                                 outHandle:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName() failed for wallet 1");
    
    // 2. Open wallet 2
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName2
                                                    config:nil
                                                 outHandle:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName() failed for wallet 2");
    
    [TestUtils cleanupStorage];
}

- (void)testOpenWalletWorksForInvalidConfig
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_open_wallet_works_for_invalid_config";
    NSString *walletName = @"indy_open_wallet_works_for_invalid_config";
    NSString *config = @"{\"field\":\"value\"}";
    NSError *ret;
    
    // 1. Create wallet
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:nil
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName() failed");
    
    // 2. Open wallet
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName
                                                    config:config
                                                 outHandle:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"WalletUtils:openWalletWithName() failed");
    
    [TestUtils cleanupStorage];
}



// MARK: - Close wallet

// WARNING: createAndOpenWallet method is a workaround to ensure that we try to close non existing walletHandle. In Rust test only closeWalletWithHandle:1 is used
- (void)testCloseWalletWorksForInvalidHandle
{
    [TestUtils cleanupStorage];
    
    IndyHandle walletHandle = 0;
    NSError *ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"testCloseWalletWorksForInvalidHandle"
                                                                           xtype:nil
                                                                          handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName() failed");
    
    [TestUtils cleanupStorage];

    ret = [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle + 1];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"WalletUtils:closeWalletWithHandle() returned wrong code for walletHandle: %d", walletHandle);
    
    [TestUtils cleanupStorage];
}

- (void)testCloseWalletWorksForTwice
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_close_wallet_works_for_twice";
    NSError *ret;
    
    // 1. create and open wallet
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
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

@end
