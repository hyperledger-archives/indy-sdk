#import <Foundation/Foundation.h>

#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"

@interface PoolHighCases : XCTestCase

@end

@implementation PoolHighCases

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

// MARK: - Create
- (void)testCreatePoolLedgerConfigWorks {
    NSString *txnFilePath = [[PoolUtils sharedInstance] createGenesisTxnFileForTestPool:[TestUtils pool]
                                                                             nodesCount:nil
                                                                            txnFilePath:nil];
    NSString *poolConfig = [[PoolUtils sharedInstance] poolConfigJsonForTxnFilePath:txnFilePath];

    NSError *res = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:[TestUtils pool]
                                                                       poolConfig:poolConfig];
    XCTAssertEqual(res.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName failed");
};

// MARK: - Open

- (void)testOpenPoolLedgerWorks {
    NSString *poolName = @"testOpenPoolLedgerWorks";

    NSString *txnFilePath = [[PoolUtils sharedInstance] createGenesisTxnFileForTestPool:poolName
                                                                             nodesCount:nil
                                                                            txnFilePath:nil];
    NSString *poolConfig = [[PoolUtils sharedInstance] poolConfigJsonForTxnFilePath:txnFilePath];

    // 1. Create pool ledger config
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                       poolConfig:poolConfig];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName() failed!");

    // 2. Open pool ledger
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName
                                              config:nil
                                         poolHandler:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::openPoolLedger() failed!");
}

- (void)testOpenPoolLedgerWorksForInvalidName {
    NSError *ret = [[PoolUtils sharedInstance] openPoolLedger:[TestUtils pool]
                                                       config:nil
                                                  poolHandler:nil];
    XCTAssertEqual(ret.code, PoolLedgerNotCreatedError, @"PoolUtils::openPoolLedger returned wrong code");
}

- (void)testOpenPoolLedgerWorksForTwice {
    NSString *poolName = @"testOpenPoolLedgerWorksForTwice";

    // 1. create pool ledger config
    NSError *ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                                        poolHandle:nil];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName() failed!");

    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName
                                              config:nil
                                         poolHandler:nil];
    XCTAssertEqual(ret.code, PoolLedgerInvalidPoolHandle, @"PoolUtils::openPoolLedger() failed second one!");
}

- (void)testOpenPoolLedgerWorksForIncompatibleProtocolVersion {
    NSError *ret = [[PoolUtils sharedInstance] setProtocolVersion:@(1)];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::setProtocolVersion() failed!");

    NSString *txnFilePath = [[PoolUtils sharedInstance] createGenesisTxnFileForTestPool:[TestUtils pool]
                                                                             nodesCount:nil
                                                                            txnFilePath:nil];
    NSString *poolConfig = [[PoolUtils sharedInstance] poolConfigJsonForTxnFilePath:txnFilePath];

    // 1. Create pool ledger config
    ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:[TestUtils pool]
                                                              poolConfig:poolConfig];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName() failed!");

    // 2. Open pool ledger
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] openPoolLedger:[TestUtils pool]
                                              config:nil
                                         poolHandler:&poolHandle];
    XCTAssertEqual(ret.code, PoolIncompatibleProtocolVersion, @"PoolUtils::openPoolLedger() failed!");
}

// MARK: - Refresh

- (void)testIndyRefreshPoolLedgerWorks {
    IndyHandle poolHandle = 0;
    NSError *ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:[TestUtils pool]
                                                                        poolHandle:&poolHandle];

    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed!");

    ret = [[PoolUtils sharedInstance] refreshPoolHandle:poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::refreshPoolHandle() failed!");

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
}

// MARK: - Close

- (void)testClosePoolLedgerWorks {
    // 1. create and open pool ledger config
    IndyHandle poolHandle = 0;
    NSError *ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:[TestUtils pool]
                                                                        poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed!");

    // 2. close pool ledger
    ret = [[PoolUtils sharedInstance] closeHandle:poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::closeHandle() failed!");
}

- (void)testClosePoolLedgerWorksForTwice {
    // 1. create and open pool ledger config
    IndyHandle poolHandle = 0;
    NSError *ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:[TestUtils pool]
                                                                        poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed!");

    // 2. close pool ledger
    ret = [[PoolUtils sharedInstance] closeHandle:poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::closeHandle() failed!");

    // 3. close pool ledger
    ret = [[PoolUtils sharedInstance] closeHandle:poolHandle];
    XCTAssertEqual(ret.code, PoolLedgerInvalidPoolHandle, @"PoolUtils::closeHandle() returned wrong code!");
}

// MARK: - Delete

- (void)testIndyDeletePoolLedgerConfigWorks {
    [TestUtils cleanupStorage];

    // 1. create and open pool ledger config
    NSString *txnFilePath = [[PoolUtils sharedInstance] createGenesisTxnFileForTestPool:[TestUtils pool]
                                                                             nodesCount:nil
                                                                            txnFilePath:nil];
    NSString *poolConfig = [[PoolUtils sharedInstance] poolConfigJsonForTxnFilePath:txnFilePath];
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:[TestUtils pool]
                                                                       poolConfig:poolConfig];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName() failed!");

    // 2. delete
    ret = [[PoolUtils sharedInstance] deletePoolWithName:[TestUtils pool]];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::deletePoolWithName() failed!");
}

- (void)testDeletePoolLedgerConfigWorksForOpened {

    // 1. create and open pool ledger config
    IndyHandle poolHandle = 0;
    NSError *ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:[TestUtils pool]
                                                                        poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed!");

    // 2. delete
    ret = [[PoolUtils sharedInstance] deletePoolWithName:[TestUtils pool]];
    XCTAssertEqual(ret.code, CommonInvalidState, @"PoolUtils::deletePoolWithName() returned wrong code!");

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
}

- (void)testDeletePoolLedgerConfigWorksForNotCreated {
    // 1. delete
    NSError *ret = [[PoolUtils sharedInstance] deletePoolWithName:[TestUtils pool]];
    XCTAssertEqual(ret.code, CommonIOError, @"PoolUtils::deletePoolWithName returned wrong code");
}

- (void)testSetProtocolVersionWorks {
    NSError *ret = [[PoolUtils sharedInstance] setProtocolVersion:[TestUtils protocolVersion]];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::setProtocolVersion() failed!");
}

- (void)testSetProtocolVersionWorksForUnsupported {
    NSError *ret = [[PoolUtils sharedInstance] setProtocolVersion:@(0)];
    XCTAssertEqual(ret.code, PoolIncompatibleProtocolVersion, @"PoolUtils::setProtocolVersion() failed!");
}

@end
