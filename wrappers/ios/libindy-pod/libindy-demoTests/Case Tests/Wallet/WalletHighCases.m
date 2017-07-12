//
//  WalletHighCases.m
//  libindy-demo
//
//  Created by Anastasia Tarasova on 14.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//


#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import <libindy/libindy.h>
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

// MARK: - Register wallet type

// MARK: - Create wallet

- (void)testCreateWalletWorks
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_create_wallet_works";
    NSString *walletName = @"indy_create_wallet_works";
    NSString *xtype = @"default";
    
    NSError *ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                               walletName:walletName
                                                                    xtype:xtype
                                                                   config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName() failed");
    
    [TestUtils cleanupStorage];
}

// TODO: Finish after registerWalletType is implemented
- (void)testCreateWalletWorksForPlugged
{
    [TestUtils cleanupStorage];
    //InmemWallet::cleanup();
    NSString *poolName = @"indy_create_wallet_works";
    NSString *walletName = @"indy_create_wallet_works";
    NSString *xtype = @"inmem";
    
    // register type
    
    // create wallet
    NSError *ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                               walletName:walletName
                                                                    xtype:xtype
                                                                   config:nil];
    XCTAssertEqual(ret.code, WalletUnknownTypeError, @"WalletUtils:createWalletWithPoolName() failed");
    
    
    //InmemWallet::cleanup();
    [TestUtils cleanupStorage];
}

- (void)testCreateWalletWorksForUnknownType
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_create_wallet_works_for_unknown_type";
    NSString *walletName = @"indy_create_wallet_works_for_unknown_type";
    NSString *xtype = @"type";
    
    NSError *ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                               walletName:walletName
                                                                    xtype:xtype
                                                                   config:nil];
    XCTAssertEqual(ret.code, WalletUnknownTypeError, @"WalletUtils:createWalletWithPoolName() returned wrong error");
    
    [TestUtils cleanupStorage];
}

- (void)testCreateWalletWorksForEmptyType
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_create_wallet_works_for_empty_type";
    NSString *walletName = @"indy_create_wallet_works_for_empty_type";
    
    NSError *ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                               walletName:walletName
                                                                    xtype:nil
                                                                   config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");
    
    [TestUtils cleanupStorage];
}

- (void)testCreateWalletWorksForConfig
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_create_wallet_works";
    NSString *walletName = @"indy_create_wallet_works";
    NSString *xtype = @"default";
    NSString *config = @"{\"freshness_time\":1000}";
    
    NSError *ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                               walletName:walletName
                                                                    xtype:xtype
                                                                   config:config];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");
    
    [TestUtils cleanupStorage];
}

// MARK: - Delete wallet

- (void)testDeleteWalletWorks
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_delete_wallet_works";
    NSString *walletName = @"indy_delete_wallet_works";
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

// TODO: Finish when InmemWallet will be implemented
- (void)testDeleteWalletWorksForPlugged
{
    [TestUtils cleanupStorage];
    //[InmemWallet cleanupStorage];
    
    NSError *ret;
    NSString *poolName = @"indy_delete_wallet_works_for_plugged";
    NSString *walletName = @"indy_delete_wallet_works_for_plugged";
    NSString *xtype = @"inmem";
    
    // 1. Register wallet type
    
    // 2. Create wallet
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:xtype
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");
    
    // 3. Delete wallet
    ret = [[WalletUtils sharedInstance] deleteWalletWithName:walletName];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:deleteWalletWithName failed");
    
    // 4. Create wallet
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:xtype
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");
    
    //[InmemWallet cleanupStorage];
    [TestUtils cleanupStorage];
}

// MARK: - Open wallet
- (void)testOpenWalletWorks
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_open_wallet_works";
    NSString *walletName = @"indy_open_wallet_works";
    NSError *ret;
    
    // 1. Create wallet
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:nil
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");
    
    // 2. Open wallet
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName
                                                    config:nil
                                                 outHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName failed");
    
    [TestUtils cleanupStorage];
}

// TODO: Finish when inmem wallet will be implemented
- (void)testOpenWalletWorksForPlugged
{
    [TestUtils cleanupStorage];
    
    NSString *poolName = @"indy_open_wallet_works_for_plugged";
    NSString *walletName = @"indy_open_wallet_works_for_plugged";
    NSString *xtype = @"inmem";
    NSError *ret;
    
    // 1. register wallet type
    
    // 2. Create wallet
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:xtype
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");
    
    // 3. Open wallet
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName
                                                    config:nil
                                                 outHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName failed");
    
    [TestUtils cleanupStorage];
}

- (void)testOpenWalletWorksForConfig
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_open_wallet_works_for_config";
    NSString *walletName = @"indy_open_wallet_works_for_config";
    NSString *config = @"{\"freshness_time\":1000}";
    NSError *ret;
    
    // 1. Create wallet
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:nil
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");
    
    // 2. Open wallet
    IndyHandle walletHandle = 0;
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
    NSString *poolName = @"indy_close_wallet_works";
    NSString *walletName = @"indy_close_wallet_works";
    NSError *ret;
    
    // 1. create wallet
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:nil
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");
    
    // 2. open wallet
    IndyHandle walletHandle;
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

- (void)testCloseWalletWorksForPlugged
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_close_wallet_works_for_plugged";
    NSString *walletName = @"indy_close_wallet_works_for_plugged";
    NSString *xtype = @"inmem";
    NSError *ret;
    
    // 1. register wallet type
    
    // 2. create wallet
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:xtype
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");
    
    // 3. open wallet
    IndyHandle walletHandle;
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName
                                                    config:nil
                                                 outHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName failed");
    
    // 4. close wallet
    ret = [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:closeWalletWithHandle failed");
    
    // 5. open wallet
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName
                                                    config:nil
                                                 outHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName failed");
    
    [TestUtils cleanupStorage];
}

// MARK: - Set seq no
- (void)testWalletSetSeqNoWorks
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_wallet_set_seqno_works";
    NSError *ret;
    
    // 1. create and open wallet
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName() failed");
    
    // 2. Create my did
    NSString *did;
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:@"{}"
                                                           outMyDid:&did
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils:createMyDidWithWalletHandle() failed");
    
    // 3. Wallet set seq no for no value
    ret = [[WalletUtils sharedInstance] walletSetSeqNo:@(1)
                                              forValue:did
                                          walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:walletSetSeqNo() failed");
    
    [TestUtils cleanupStorage];
}

// TODO: Finish when inmem wallet will be implemented
- (void)testWalletSetSeqNoWorksForPlugged
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_wallet_set_seqno_works_for_plugged";
    NSString *xtype = @"inmem";
    NSError *ret;
    
    // 1. register wallet type
    
    // 2. create and open wallet
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName() failed");
    
    // 3. Create my did
    NSString *did;
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:@"{}"
                                                           outMyDid:&did
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils:createMyDidWithWalletHandle() failed");
    
    // 4. Wallet set seq no for no value
    ret = [[WalletUtils sharedInstance] walletSetSeqNo:@(1)
                                              forValue:did
                                          walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:walletSetSeqNo() failed");
    
    [TestUtils cleanupStorage];
}


@end
