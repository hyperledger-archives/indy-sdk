//
//  WalletHighCases.m
//  Indy-demo
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

- (void)testRegisterWalletTypeWorks
{
    [TestUtils cleanupStorage];
   [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
    
    NSString *xtype = @"keychain";
    
    NSError *ret = [[WalletUtils sharedInstance] registerWalletType:xtype];
    
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:@"pool1"
                                                      walletName:@"wallet1"
                                                           xtype:xtype
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName() failed");
    
   [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
    [TestUtils cleanupStorage];
}

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

- (void)testCreateWalletWorksForPlugged
{
    [TestUtils cleanupStorage];
   [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
    
    NSError *ret;
    NSString *poolName = @"indy_create_wallet_works";
    NSString *walletName = @"indy_create_wallet_works";
    NSString *xtype = @"keychain";
    
    // register type
    
    ret = [[WalletUtils sharedInstance] registerWalletType:xtype];
    // TODO Success or already registered: XCTAssertEqual(ret.code, Success, @"WalletUtils:registerWalletType() failed");
    
    // create wallet
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:xtype
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName() failed");
    
    
   [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
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

- (void)testDeleteWalletWorksForClosed
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_delete_wallet_works_for_closed";
    NSString *walletName = @"indy_delete_wallet_works_for_closed";
    NSError *ret;
    
    // 1. create wallet
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:nil
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");
    
    // 2. open wallet
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName
                                                    config:nil
                                                 outHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName failed");
    
    // 3. close wallet
    ret = [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:closeWalletWithHandle failed");
    
    // 4. delete wallet
    ret = [[WalletUtils sharedInstance] deleteWalletWithName:walletName];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:deleteWalletWithName failed");
    
    // 5. create wallet
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
    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
    
    NSError *ret;
    NSString *poolName = @"indy_delete_wallet_works_for_plugged";
    NSString *walletName = @"indy_delete_wallet_works_for_plugged";
    NSString *xtype = @"keychain";
    
    // 1. Register wallet type
    
    ret = [[WalletUtils sharedInstance] registerWalletType:xtype];
    
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
    
    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
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
   [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
    
    NSString *poolName = @"indy_open_wallet_works_for_plugged";
    NSString *walletName = @"indy_open_wallet_works_for_plugged";
    NSString *xtype = @"keychain";
    NSError *ret;
    
    // 1. register wallet type
    ret = [[WalletUtils sharedInstance] registerWalletType:xtype];
    
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
    
   [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
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
    ret = [[WalletUtils sharedInstance] registerWalletType:xtype];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:registerWalletType failed");
    
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

@end
