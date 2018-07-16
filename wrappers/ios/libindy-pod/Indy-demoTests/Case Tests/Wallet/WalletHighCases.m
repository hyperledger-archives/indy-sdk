
#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"

@interface WalletHighCases : XCTestCase

@end

@implementation WalletHighCases

- (void)setUp {
    [super setUp];
    [TestUtils cleanupStorage];

    NSError *ret = [[PoolUtils sharedInstance] setProtocolVersion:[TestUtils protocolVersion]];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::setProtocolVersion() failed!");

    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [TestUtils cleanupStorage];
    [super tearDown];
}

// MARK: - Register wallet type

//- (void)testRegisterWalletTypeWorks {
//    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
//
//    NSString *xtype = @"keychain";
//
//    NSError *ret = [[WalletUtils sharedInstance] registerWalletType:xtype];
//
//    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:[TestUtils pool]
//                                                      walletName:[TestUtils wallet]
//                                                           xtype:xtype
//                                                          config:nil];
//    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName() failed");
//
//    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
//}

// MARK: - Create wallet

- (void)testCreateWalletWorks {
    NSError *ret = [[WalletUtils sharedInstance] createWalletWithConfig:[TestUtils walletConfig]];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName() failed");
}

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

- (void)testCreateWalletWorksForUnknownType {
    NSString *walletConfig = @"{\"id\":\"wallet_1\", \"storage_type\":\"unknown_type\"}";

    NSError *ret = [[WalletUtils sharedInstance] createWalletWithConfig:walletConfig];
    XCTAssertEqual(ret.code, WalletUnknownTypeError, @"WalletUtils:createWalletWithPoolName() returned wrong error");
}

// MARK: - Delete wallet

- (void)testDeleteWalletWorks {
    // 1. Create wallet
    NSError *ret = [[WalletUtils sharedInstance] createWalletWithConfig:[TestUtils walletConfig]];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");

    // 2. Delete wallet
    ret = [[WalletUtils sharedInstance] deleteWalletWithConfig:[TestUtils walletConfig]];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:deleteWalletWithName failed");

    // 3. Create wallet
    ret = [[WalletUtils sharedInstance] createWalletWithConfig:[TestUtils walletConfig]];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");
}

- (void)testDeleteWalletWorksForClosed {
    // 1. create wallet
    NSError *ret = [[WalletUtils sharedInstance] createWalletWithConfig:[TestUtils walletConfig]];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");

    // 2. open wallet
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] openWalletWithConfig:[TestUtils walletConfig]
                                                   outHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName failed");

    // 3. close wallet
    ret = [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:closeWalletWithHandle failed");

    // 4. delete wallet
    ret = [[WalletUtils sharedInstance] deleteWalletWithConfig:[TestUtils walletConfig]];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:deleteWalletWithName failed");

    // 5. create wallet
    ret = [[WalletUtils sharedInstance] createWalletWithConfig:[TestUtils walletConfig]];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");
}

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

- (void)testDeleteWalletWorksForUnknown {
    NSError *ret = [[WalletUtils sharedInstance] deleteWalletWithConfig:[TestUtils walletConfig]];
    XCTAssertEqual(ret.code, WalletNotFoundError, @"WalletUtils:deleteWalletWithName() returned wrong error");
}

// MARK: - Open wallet

- (void)testOpenWalletWorks {
    NSString *walletConfig = @"{\"id\":\"indy_open_wallet_works\"}";

    // 1. Create wallet
    NSError *ret = [[WalletUtils sharedInstance] createWalletWithConfig:walletConfig];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");

    // 2. Open wallet
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] openWalletWithConfig:walletConfig
                                                   outHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName failed");
}

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
//                                                      walletName:walletName
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

- (void)testOpenWalletWorksForNotCreatedWallet {
    NSError *ret = [[WalletUtils sharedInstance] openWalletWithConfig:[TestUtils walletConfig]
                                                            outHandle:nil];
    XCTAssertEqual(ret.code, WalletNotFoundError, @"WalletUtils:openWalletWithName() failed");
}

// MARK: - Close wallet

- (void)testCloseWalletWorks {
    // 1. create wallet
    NSError *ret = [[WalletUtils sharedInstance] createWalletWithConfig:[TestUtils walletConfig]];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");

    // 2. open wallet
    IndyHandle walletHandle;
    ret = [[WalletUtils sharedInstance] openWalletWithConfig:[TestUtils walletConfig]
                                                   outHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName failed");

    // 3. close wallet
    ret = [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:closeWalletWithHandle failed");

    // 4. open wallet
    ret = [[WalletUtils sharedInstance] openWalletWithConfig:[TestUtils walletConfig]
                                                   outHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName failed");

    // 5. close wallet
    ret = [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:closeWalletWithHandle failed");
}

//- (void)testCloseWalletWorksForPlugged {
//    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
//
//    // 1. register wallet type
//    [[WalletUtils sharedInstance] registerWalletType:[TestUtils keychainType]];
//
//    // 2. create wallet
//    NSError *ret = [[WalletUtils sharedInstance] createWalletWithPoolName:[TestUtils pool]
//                                                               walletName:[TestUtils wallet]
//                                                                    xtype:[TestUtils keychainType]
//                                                                   config:nil];
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
//                                                    config:nil
//                                                 outHandle:&walletHandle];
//    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName failed");
//
//    // 6. close wallet
//    ret = [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
//    XCTAssertEqual(ret.code, Success, @"WalletUtils:closeWalletWithHandle failed");
//
//    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
//}

- (void)testExportWalletWorks {
    // 1. create and open wallet
    IndyHandle walletHandle;
    NSError *ret = [[WalletUtils sharedInstance] createAndOpenWalletWithHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndOpenWalletWithPoolName() failed");

    // 2. create did
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:@"{}"
                                                        outMyDid:nil
                                                     outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");

    // 3. export wallet
    NSString *exportFile = [TestUtils tmpFilePathAppending:@"export_wallet"];

    NSString *exportConfig = [[AnoncredsUtils sharedInstance] toJson:@{
            @"path": exportFile,
            @"key": @"export_key"
    }];

    ret = [[WalletUtils sharedInstance] exportWalletWithHandle:walletHandle
                                              exportConfigJson:exportConfig];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndOpenWalletWithPoolName() failed");

    // 4. check file exists
    XCTAssertEqual(YES, [[NSFileManager defaultManager] fileExistsAtPath:exportFile], @"FILE NOT FOUND");

    // 5. close wallet
    ret = [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:closeWalletWithHandle failed");

    [[NSFileManager defaultManager] removeItemAtPath:exportFile error:nil];
}

- (void)testImportWalletWorks {
    // create wallet
    NSError *ret = [[WalletUtils sharedInstance] createWalletWithConfig:[TestUtils walletConfig]];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");

    // open wallet
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] openWalletWithConfig:[TestUtils walletConfig]
                                                   outHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName failed");

    // create did
    NSString *did;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:@"{}"
                                                        outMyDid:&did
                                                     outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");

    // get key for did before export
    NSString *keyForDidBeforeExport;
    ret = [[DidUtils sharedInstance] keyForLocalDid:did
                                       walletHandle:walletHandle
                                                key:&keyForDidBeforeExport];
    XCTAssertEqual(ret.code, Success, @"DidUtils::keyForDid() failed");

    // 3. export wallet
    NSString *exportFile = [TestUtils tmpFilePathAppending:@"export_wallet"];

    NSString *exportConfig = [[AnoncredsUtils sharedInstance] toJson:@{
            @"path": exportFile,
            @"key": @"export_key"
    }];

    ret = [[WalletUtils sharedInstance] exportWalletWithHandle:walletHandle
                                              exportConfigJson:exportConfig];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndOpenWalletWithPoolName() failed");

    // close wallet
    ret = [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:closeWalletWithHandle failed");

    // delete wallet
    ret = [[WalletUtils sharedInstance] deleteWalletWithConfig:[TestUtils walletConfig]];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:deleteWalletWithName failed");

    // import wallet
    ret = [[WalletUtils sharedInstance] importWalletWithConfig:[TestUtils walletConfig]
                                              importConfigJson:exportConfig];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName() failed");

    // open wallet
    ret = [[WalletUtils sharedInstance] openWalletWithConfig:[TestUtils walletConfig]
                                                   outHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName failed");

    // get key for did after import
    NSString *keyForDidAfterImport;
    ret = [[DidUtils sharedInstance] keyForLocalDid:did
                                       walletHandle:walletHandle
                                                key:&keyForDidAfterImport];
    XCTAssertEqual(ret.code, Success, @"DidUtils::keyForDid() failed");

    // compare keys
    XCTAssertTrue([keyForDidBeforeExport isEqualToString:keyForDidAfterImport]);

    // close wallet
    ret = [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:closeWalletWithHandle failed");

    [[NSFileManager defaultManager] removeItemAtPath:exportFile error:nil];
}

@end
