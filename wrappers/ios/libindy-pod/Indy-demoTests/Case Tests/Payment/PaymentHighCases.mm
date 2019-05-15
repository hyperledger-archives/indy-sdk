#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import "PaymentUtils.h"

@interface PaymentHighCases : XCTestCase

@end

@implementation PaymentHighCases {
    IndyHandle walletHandle;
    NSError *ret;
}

- (void)setUp {
    [super setUp];
    [TestUtils cleanupStorage];

    ret = [[PoolUtils sharedInstance] setProtocolVersion:[TestUtils protocolVersion]];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::setProtocolVersion() failed!");

    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithHandle:&walletHandle];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];

    [TestUtils cleanupStorage];
    [super tearDown];
}

NSString *paymentMethod = @"null";
NSString *paymentAddress = @"pay:null:test";
NSString *inputs = @"[\"pay:null:1\", \"pay:null:2\"]";
NSString *outputs = @"[{\"recipient\": \"pay:null:1\", \"amount\":1}, {\"recipient\": \"pay:null:2\", \"amount\":2}]";
NSString *fees = @"{\"txnType1\":1, \"txnType2\":2}";
NSString *incompatibleInputs = @"[\"pay:PAYMENT_METHOD_1:1\", \"pay:PAYMENT_METHOD_2:1\"]";
NSString *receipt = @"pay:null:0_PqVjwJC42sxCTJp";

// MARK: - Create Payment Address

- (void)testCreatePaymentAddressWorks {
    ret = [[PaymentUtils sharedInstance] createPaymentAddressForMethod:paymentMethod
                                                          walletHandle:walletHandle
                                                                config:@"{}"
                                                        paymentAddress:nil];
    XCTAssertEqual(ret.code, PaymentUnknownMethodError);
}

// MARK: - Create Payment Address

- (void)testListPaymentAddressWorks {
    NSString *paymentAddresses;
    ret = [[PaymentUtils sharedInstance] listPaymentAddresses:walletHandle
                                             paymentAddresses:&paymentAddresses];
    XCTAssertEqual(ret.code, Success);

    NSDictionary *result = [NSDictionary fromString:paymentAddresses];
    XCTAssertEqual(0, [result count]);
}

// MARK: - Add Request Fees

- (void)testAddRequestFeesWorks {
    ret = [[PaymentUtils sharedInstance] addFeesToRequest:@"{}"
                                             walletHandle:walletHandle
                                             submitterDid:[TestUtils trusteeDid]
                                               inputsJson:inputs
                                              outputsJson:outputs
                                                    extra:nil
                                      requestWithFeesJson:nil
                                            paymentMethod:nil];
    XCTAssertEqual(ret.code, PaymentUnknownMethodError);
}

- (void)testAddRequestFeesWorksForExtra {
    ret = [[PaymentUtils sharedInstance] addFeesToRequest:@"{}"
                                             walletHandle:walletHandle
                                             submitterDid:[TestUtils trusteeDid]
                                               inputsJson:inputs
                                              outputsJson:outputs
                                                    extra:@"Extra data"
                                      requestWithFeesJson:nil
                                            paymentMethod:nil];
    XCTAssertEqual(ret.code, PaymentUnknownMethodError);
}

- (void)testAddRequestFeesWorksForSeveralMethods {
    ret = [[PaymentUtils sharedInstance] addFeesToRequest:@"{}"
                                             walletHandle:walletHandle
                                             submitterDid:[TestUtils trusteeDid]
                                               inputsJson:incompatibleInputs
                                              outputsJson:@"[]"
                                                    extra:nil
                                      requestWithFeesJson:nil
                                            paymentMethod:nil];
    XCTAssertEqual(ret.code, PaymentIncompatibleMethodsError);
}

// MARK: - Parse Response With Fees

- (void)testParseResponseWithFeesWorks {
    ret = [[PaymentUtils sharedInstance] parseResponseWithFees:@"{}"
                                                 paymentMethod:paymentMethod
                                                  receiptsJson:nil];
    XCTAssertEqual(ret.code, PaymentUnknownMethodError);
}

// MARK: - Build Get Sources Request

- (void)testBuildGetPaymentSourcesRequestWorks {
    ret = [[PaymentUtils sharedInstance] buildGetPaymentSourcesRequest:walletHandle
                                                          submitterDid:[TestUtils trusteeDid]
                                                        paymentAddress:paymentAddress
                                                     getSourcesTxnJson:nil
                                                         paymentMethod:nil];
    XCTAssertEqual(ret.code, PaymentUnknownMethodError);
}

// MARK: - Parse Get Sources Response

- (void)testParseGetPaymentSourcesResponseWorks {
    ret = [[PaymentUtils sharedInstance] parseGetPaymentSourcesResponse:@"{}"
                                                          paymentMethod:paymentMethod
                                                            sourcesJson:nil];
    XCTAssertEqual(ret.code, PaymentUnknownMethodError);
}

// MARK: - Build Payment Request

- (void)testBuildPaymentRequestWorks {
    ret = [[PaymentUtils sharedInstance] buildPaymentRequest:walletHandle
                                                submitterDid:[TestUtils trusteeDid]
                                                  inputsJson:inputs
                                                 outputsJson:outputs
                                                       extra:nil
                                              paymentReqJson:nil
                                               paymentMethod:nil];
    XCTAssertEqual(ret.code, PaymentUnknownMethodError);
}

// MARK: - Parse Payment Response

- (void)testParsePaymentResponseWorks {
    ret = [[PaymentUtils sharedInstance] parsePaymentResponse:@"{}"
                                                paymentMethod:paymentMethod
                                                 receiptsJson:nil];
    XCTAssertEqual(ret.code, PaymentUnknownMethodError);
}

// MARK: - Prepare Payment Extra With Acceptance Data

- (void)testPreparePaymentExtraWithAcceptanceDataWorks {
    NSDictionary *extra = @{
            @"data": @"some extra data"
    };

    NSString *extraWithAcceptance;
    ret = [[PaymentUtils sharedInstance] preparePaymentExtraWithAcceptanceData:[NSDictionary toString:extra]
                                                                          text:@"some agreement text"
                                                                       version:@"1.0.0"
                                                                     taaDigest:@"050e52a57837fff904d3d059c8a123e3a04177042bf467db2b2c27abd8045d5e"
                                                                   accMechType:@"acceptance type 1"
                                                              timeOfAcceptance:@(123456789)
                                                           extraWithAcceptance:&extraWithAcceptance];
    XCTAssertEqual(ret.code, Success, @"PaymentUtils::preparePaymentExtraWithAcceptanceData() failed!");
    NSDictionary *expectedExtra = @{
            @"data": @"some extra data",
            @"taaAcceptance": @{
                    @"mechanism": @"acceptance type 1",
                    @"taaDigest": @"050e52a57837fff904d3d059c8a123e3a04177042bf467db2b2c27abd8045d5e",
                    @"time": @(123456789),
            }
    };

    NSDictionary *actualExtra = [NSDictionary fromString:extraWithAcceptance];

    XCTAssertTrue([expectedExtra isEqualToDictionary:actualExtra], @"Wrong Extra Json!");
}

// MARK: - Build Mint Request

- (void)testBuildMintRequestWorks {
    ret = [[PaymentUtils sharedInstance] buildMintRequest:walletHandle
                                             submitterDid:[TestUtils trusteeDid]
                                              outputsJson:outputs
                                                    extra:nil
                                              mintReqJson:nil
                                            paymentMethod:nil];
    XCTAssertEqual(ret.code, PaymentUnknownMethodError);
}

// MARK: - Build Set Txn Fees Request

- (void)testBuildSetTxnFeesRequestWorks {
    ret = [[PaymentUtils sharedInstance] buildSetTxnFeesRequest:walletHandle
                                                   submitterDid:[TestUtils trusteeDid]
                                                  paymentMethod:paymentMethod
                                                       feesJson:fees
                                              setTxnFeesReqJson:nil];
    XCTAssertEqual(ret.code, PaymentUnknownMethodError);
}

// MARK: - Build Get Txn Fees Request

- (void)testBuildGetTxnFeesRequestWorks {
    ret = [[PaymentUtils sharedInstance] buildGetTxnFeesRequest:walletHandle
                                                   submitterDid:[TestUtils trusteeDid]
                                                  paymentMethod:paymentMethod
                                              getTxnFeesReqJson:nil];
    XCTAssertEqual(ret.code, PaymentUnknownMethodError);
}

// MARK: - Parse Get Txn Fees Response

- (void)testParseGetTxnFeesResponseWorks {
    ret = [[PaymentUtils sharedInstance] parseGetTxnFeesResponse:@"{}"
                                                   paymentMethod:paymentMethod
                                                        feesJson:nil];
    XCTAssertEqual(ret.code, PaymentUnknownMethodError);
}

// MARK: - Build Verify Payment Request

- (void)testBuildVerifyPaymentRequestWorks {
    ret = [[PaymentUtils sharedInstance] buildVerifyPaymentRequest:walletHandle
                                                      submitterDid:[TestUtils trusteeDid]
                                                           receipt:receipt
                                                     verifyReqJson:nil
                                                     paymentMethod:nil];
    XCTAssertEqual(ret.code, PaymentUnknownMethodError);
}

// MARK: - Parse Verify Payment Response

- (void)testParseVerifyPaymentResponseWorks {
    ret = [[PaymentUtils sharedInstance] parseVerifyPaymentResponse:@"{}"
                                                      paymentMethod:paymentMethod
                                                    receiptInfoJson:nil];
    XCTAssertEqual(ret.code, PaymentUnknownMethodError);
}

@end
