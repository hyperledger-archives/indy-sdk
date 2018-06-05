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

@interface DefaultWallet : XCTestCase

@end

@implementation DefaultWallet

- (void)setUp {
    [super setUp];
    [TestUtils cleanupStorage];
    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
    [TestUtils cleanupStorage];
    [super tearDown];
}

// MARK: - Register wallet type

- (void)testRegisterWalletTypeWorks {
    NSString *xtype = @"keychain";

    NSError *ret = [[WalletUtils sharedInstance] registerWalletType:xtype];

    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:[TestUtils pool]
                                                      walletName:[TestUtils wallet]
                                                           xtype:xtype
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName() failed");
}

// MARK: - Create wallet

//- (void)testCreateWalletWorksForPlugged {
//    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
//
//    // register type
//    NSError *ret = [[WalletUtils sharedInstance] registerWalletType:[TestUtils keychainType]];
//
//    // create wallet
//    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:[TestUtils pool]
//                                                      walletName:[TestUtils wallet]
//                                                           xtype:[TestUtils keychainType]
//                                                          config:nil];
//    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName() failed");
//
//    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
//}

// MARK: - Delete wallet

//- (void)testDeleteWalletWorksForPlugged {
//    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
//
//    // 1. Register wallet type
//    NSError *ret = [[WalletUtils sharedInstance] registerWalletType:[TestUtils keychainType]];
//
//    // 2. Create wallet
//    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:[TestUtils pool]
//                                                      walletName:[TestUtils wallet]
//                                                           xtype:[TestUtils keychainType]
//                                                          config:nil];
//    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");
//
//    // 3. Delete wallet
//    ret = [[WalletUtils sharedInstance] deleteWalletWithName:[TestUtils wallet]];
//    XCTAssertEqual(ret.code, Success, @"WalletUtils:deleteWalletWithName failed");
//
//    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
//}

// MARK: - Open wallet

//- (void)testOpenWalletWorksForPlugged {
//    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
//
//    NSString *walletName = @"indy_open_wallet_works_for_plugged";
//
//    // 1. register wallet type
//    NSError *ret = [[WalletUtils sharedInstance] registerWalletType:[TestUtils keychainType]];
//
//    // 2. Create wallet
//    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:[TestUtils pool]
//                                                     walletName:walletName
//                                                           xtype:[TestUtils keychainType]
//                                                          config:nil];
//    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");
//
//    // 3. Open wallet
//    IndyHandle walletHandle = 0;
//    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName
//                                                    config:nil
//                                                 outHandle:&walletHandle];
//    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName failed");
//
//    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
//}

// MARK: - Close wallet

//- (void)testCloseWalletWorksForPlugged {
//    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
//
//    // 1. register wallet type
//    NSError *ret = [[WalletUtils sharedInstance] registerWalletType:[TestUtils keychainType]];
//    XCTAssertEqual(ret.code, Success, @"WalletUtils:registerWalletType failed");
//
//    // 2. create wallet
//    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:[TestUtils pool]
//                                                      walletName:[TestUtils wallet]
//                                                           xtype:[TestUtils keychainType]
//                                                          config:nil];
//    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");
//
//    // 3. open wallet
//    IndyHandle walletHandle;
//    ret = [[WalletUtils sharedInstance] openWalletWithName:[TestUtils wallet]
//                                                    config:nil
//                                                 outHandle:&walletHandle];
//    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName failed");
//
//    // 4. close wallet
//    ret = [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
//    XCTAssertEqual(ret.code, Success, @"WalletUtils:closeWalletWithHandle failed");
//
//    // 5. open wallet
//    ret = [[WalletUtils sharedInstance] openWalletWithName:[TestUtils wallet]
//                                                   config:nil
//                                                 outHandle:&walletHandle];
//    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName failed");
//
//    // 6. close wallet
//   ret = [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
//    XCTAssertEqual(ret.code, Success, @"WalletUtils:closeWalletWithHandle failed");
//
//    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
//}

@end
