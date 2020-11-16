#import <XCTest/XCTest.h>
#import <Indy/Indy.h>
#import "TestUtils.h"
#import "WalletUtils.h"
#import "AnoncredsUtils.h"

@interface ProverDeleteCredentialCases : XCTestCase

@end

@implementation ProverDeleteCredentialCases {
    NSError *ret;
}

- (void)setUp {
    [super setUp];

    ret = [[PoolUtils sharedIAnoncredsHighCases.m nstance] setProtocolVersion:[TestUtils protocolVersion]];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::setProtocolVersion() failed!");
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    [AnoncredsUtils clearInstance];
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

// MARK: Prover delete credentials

- (void)testProverDeleteCredentialsWorks {
    IndyHandle walletHandle = 0;

    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. delete credential
    ret = [[AnoncredsUtils sharedInstance] proverDeleteCredentialsWithId:[[AnoncredsUtils sharedInstance] credentialId3]
                                                            walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverDeleteCredentialsWithId failed");

    // 3. get credential
    NSString *credentialJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetCredentialWithId:[[AnoncredsUtils sharedInstance] credentialId3]
                                                        walletHandle:walletHandle
                                                      credentialJson:&credentialJson];
    XCTAssertEqual(ret.code, WalletItemNotFound, @"AnoncredsUtils::proverGetCredentialsForWalletHandle credential available even though it was deleted");

}

- (void)testProverDeleteCredentialsWorksForNonexistingId {
    IndyHandle walletHandle = 0;

    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. delete non-existing credential
    ret = [[AnoncredsUtils sharedInstance] proverDeleteCredentialsWithId:[[[AnoncredsUtils sharedInstance] credentialId3] stringByAppendingString:@"a"]
                                                            walletHandle:walletHandle];
    XCTAssertEqual(ret.code, WalletItemNotFound, @"AnoncredsUtils::proverDeleteCredentialsWithId succesfully deleted non-xisting credential");

}

@end
