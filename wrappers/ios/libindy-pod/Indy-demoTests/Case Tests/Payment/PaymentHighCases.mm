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
NSString *outputs = @"[{\"paymentAddress\": \"pay:null:1\", \"amount\":1, \"extra\":\"1\"}, {\"paymentAddress\": \"pay:null:2\", \"amount\":2, \"extra\":\"2\"}]";
NSString *fees = @"{\"txnType1\":1, \"txnType2\":2}";
NSString *incompatibleInputs = @"[\"pay:PAYMENT_METHOD_1:1\", \"pay:PAYMENT_METHOD_2:1\"]";

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
                                      requestWithFeesJson:nil
                                            paymentMethod:nil];
    XCTAssertEqual(ret.code, PaymentIncompatibleMethodsError);
}

// MARK: - Parse Response With Fees

- (void)testParseResponseWithFeesWorks {
    ret = [[PaymentUtils sharedInstance] parseResponseWithFees:@"{}"
                                                 paymentMethod:paymentMethod
                                                      utxoJson:nil];
    XCTAssertEqual(ret.code, PaymentUnknownMethodError);
}

// MARK: - Build Get UTXO Request

- (void)testBuildGetUtxoRequestWorks {
    ret = [[PaymentUtils sharedInstance] buildGetUtxoRequest:walletHandle
                                                submitterDid:[TestUtils trusteeDid]
                                              paymentAddress:paymentAddress
                                              getUtxoTxnJson:nil
                                               paymentMethod:nil];
    XCTAssertEqual(ret.code, PaymentUnknownMethodError);
}

// MARK: - Parse Get UTXO Response

- (void)testParseGetUtxoResponseWorks {
    ret = [[PaymentUtils sharedInstance] parseGetUtxoResponse:@"{}"
                                                paymentMethod:paymentMethod
                                                     utxoJson:nil];
    XCTAssertEqual(ret.code, PaymentUnknownMethodError);
}

// MARK: - Build Payment Request

- (void)testBuildPaymentRequestWorks {
    ret = [[PaymentUtils sharedInstance] buildPaymentRequest:walletHandle
                                                submitterDid:[TestUtils trusteeDid]
                                                  inputsJson:inputs
                                                 outputsJson:outputs
                                              paymentReqJson:nil
                                               paymentMethod:nil];
    XCTAssertEqual(ret.code, PaymentUnknownMethodError);
}

// MARK: - Parse Payment Response

- (void)testParsePaymentResponseWorks {
    ret = [[PaymentUtils sharedInstance] parsePaymentResponse:@"{}"
                                                paymentMethod:paymentMethod
                                                     utxoJson:nil];
    XCTAssertEqual(ret.code, PaymentUnknownMethodError);
}

// MARK: - Build Mint Request

- (void)testBuildMintRequestWorks {
    ret = [[PaymentUtils sharedInstance] buildMintRequest:walletHandle
                                             submitterDid:[TestUtils trusteeDid]
                                              outputsJson:outputs
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

@end
