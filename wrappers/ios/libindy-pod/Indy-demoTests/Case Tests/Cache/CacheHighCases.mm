#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "CacheUtils.h"
#import "TestUtils.h"

@interface CacheHighCases : XCTestCase

@end

@implementation CacheHighCases {
    IndyHandle poolHandle;
    IndyHandle walletHandle;
    NSError *ret;
}

- (void)setUp {
    [super setUp];
    [TestUtils cleanupStorage];

    ret = [[PoolUtils sharedInstance] setProtocolVersion:[TestUtils protocolVersion]];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::setProtocolVersion() failed!");

    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:[TestUtils pool]
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed");

    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];

    [TestUtils cleanupStorage];
    [super tearDown];
}

- (void)testSchemaCacheWorks {
    // 1. obtain did
    NSString *myDid;
    NSString *myVerkey;
    [[DidUtils sharedInstance] createAndStoreAndPublishDidWithWalletHandle:walletHandle
                                                                poolHandle:poolHandle
                                                                       did:&myDid
                                                                    verkey:&myVerkey];

    // 2. Build schema request
    NSString *schemaId;
    NSString *schemaJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateSchemaWithName:[TestUtils gvtSchemaName]
                                                              version:[TestUtils schemaVersion]
                                                                attrs:[TestUtils gvtSchemaAttrs]
                                                            issuerDID:myDid
                                                             schemaId:&schemaId
                                                           schemaJson:&schemaJson];

    NSString *schemaRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildSchemaRequestWithSubmitterDid:myDid
                                                                      data:schemaJson
                                                                resultJson:&schemaRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildSchemaRequest() failed");

    // 3. Sign and submit schema request
    NSString *schemaResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:schemaRequest
                                                           outResponseJson:&schemaResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");

    // 4. get schema
    ret = [[CacheUtils sharedInstance] getSchema:poolHandle
                                    walletHandle:walletHandle
                                    submitterDid:myDid
                                              id:schemaId
                                     optionsJson:@"{\"noCache\":false, \"noUpdate\":false, \"noStore\":false, \"minFresh\": -1}"
                                      schemaJson:&schemaJson];

    XCTAssertEqual(ret.code, Success, @"CacheUtils::getSchema() failed");

    // 5. purge schema cache
    ret = [[CacheUtils sharedInstance] purgeSchemaCache:walletHandle
                                            optionsJson:@"{\"maxAge\":-1}"];

    XCTAssertEqual(ret.code, Success, @"CacheUtils::purgeSchemaCache() failed");
}

- (void)testCredDefCachesWorks {
    // 1. obtain did
    NSString *myDid;
    NSString *myVerkey;
    [[DidUtils sharedInstance] createAndStoreAndPublishDidWithWalletHandle:walletHandle
                                                                poolHandle:poolHandle
                                                                       did:&myDid
                                                                    verkey:&myVerkey];

    // 2. Build schema request
    NSString *schemaId;
    NSString *schemaJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateSchemaWithName:[TestUtils gvtSchemaName]
                                                              version:[TestUtils schemaVersion]
                                                                attrs:[TestUtils gvtSchemaAttrs]
                                                            issuerDID:myDid
                                                             schemaId:&schemaId
                                                           schemaJson:&schemaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateSchemaForIssuerDID failed");

    NSString *schemaRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildSchemaRequestWithSubmitterDid:myDid
                                                                      data:schemaJson
                                                                resultJson:&schemaRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildSchemaRequest() failed");

    // 3. Sign and submit schema request
    NSString *schemaResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:schemaRequest
                                                           outResponseJson:&schemaResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");

    // 4. Get schema
    ret = [[CacheUtils sharedInstance] getSchema:poolHandle
                                    walletHandle:walletHandle
                                    submitterDid:myDid
                                              id:schemaId
                                     optionsJson:@"{\"noCache\":false, \"noUpdate\":false, \"noStore\":false, \"minFresh\": -1}"
                                      schemaJson:&schemaJson];

    XCTAssertEqual(ret.code, Success, @"CacheUtils::getSchema() failed");

    // 5. Create credential definition
    NSString *credentialDefId;
    NSString *credentialDefJSON;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateAndStoreCredentialDefForSchema:schemaJson
                                                                            issuerDID:myDid
                                                                                  tag:[TestUtils tag]
                                                                                 type:nil
                                                                           configJSON:[[AnoncredsUtils sharedInstance] defaultCredentialDefConfig]
                                                                         walletHandle:walletHandle
                                                                            credDefId:&credentialDefId
                                                                          credDefJson:&credentialDefJSON];
    XCTAssertEqual(ret.code, Success, @"issuerCreateCredentialDefinitionWithWalletHandle failed");

    // 6. Build credential def request
    NSString *credDefRequestJson;
    ret = [[LedgerUtils sharedInstance] buildCredDefRequestWithSubmitterDid:myDid
                                                                       data:credentialDefJSON
                                                                 resultJson:&credDefRequestJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::buildCredDefRequestWithSubmitterDid() failed");

    // 7. Sign and submit credential def request
    NSString *credDefResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:credDefRequestJson
                                                           outResponseJson:&credDefResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() returned not Success");

    // 8. Get credential definition
    ret = [[CacheUtils sharedInstance] getCredDef:poolHandle
                                     walletHandle:walletHandle
                                     submitterDid:myDid
                                               id:credentialDefId
                                      optionsJson:@"{\"noCache\":false, \"noUpdate\":false, \"noStore\":false, \"minFresh\": -1}"
                                      credDefJson:&credentialDefJSON];

    XCTAssertEqual(ret.code, Success, @"CacheUtils::getCredDef() failed");

    // 9. purge credential definition cache
    ret = [[CacheUtils sharedInstance] purgeCredDefCache:walletHandle
                                             optionsJson:@"{\"maxAge\":-1}"];

    XCTAssertEqual(ret.code, Success, @"CacheUtils::purgeCredDefCache() failed");
}

@end
