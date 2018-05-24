#import "PaymentUtils.h"
#import <Indy/Indy.h>
#import "TestUtils.h"
#import "WalletUtils.h"

@implementation PaymentUtils

+ (PaymentUtils *)sharedInstance {
    static PaymentUtils *instance = nil;
    static dispatch_once_t dispatch_once_block;

    dispatch_once(&dispatch_once_block, ^{
        instance = [PaymentUtils new];
    });

    return instance;
}


- (NSError *)createPaymentAddressForMethod:(NSString *)paymentMethod
                              walletHandle:(IndyHandle)walletHandle
                                    config:(NSString *)config
                            paymentAddress:(NSString **)paymentAddress {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *payAddress = nil;

    [IndyPayment createPaymentAddressForMethod:paymentMethod
                                  walletHandle:walletHandle
                                        config:config
                                    completion:^(NSError *error, NSString *res) {
                                        err = error;
                                        payAddress = res;
                                        [completionExpectation fulfill];
                                    }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (paymentAddress) {*paymentAddress = payAddress;}

    return err;
}

- (NSError *)listPaymentAddresses:(IndyHandle)walletHandle
                 paymentAddresses:(NSString **)paymentAddresses {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *payAddresses = nil;

    [IndyPayment listPaymentAddresses:walletHandle
                           completion:^(NSError *error, NSString *res) {
                               err = error;
                               payAddresses = res;
                               [completionExpectation fulfill];
                           }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (paymentAddresses) {*paymentAddresses = payAddresses;}

    return err;
}

- (NSError *)addFeesToRequest:(NSString *)requestJson
                 walletHandle:(IndyHandle)walletHandle
                 submitterDid:(NSString *)submitterDid
                   inputsJson:(NSString *)inputsJson
                  outputsJson:(NSString *)outputsJson
          requestWithFeesJson:(NSString **)requestWithFeesJson
                paymentMethod:(NSString **)paymentMethod {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outReq = nil;
    __block NSString *outPayMethod = nil;

    [IndyPayment addFeesToRequest:requestJson
                     walletHandle:walletHandle
                     submitterDid:submitterDid
                       inputsJson:inputsJson
                      outputsJson:outputsJson
                       completion:^(NSError *error, NSString *req, NSString *method) {
                           err = error;
                           outReq = req;
                           outPayMethod = method;
                           [completionExpectation fulfill];
                       }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (requestWithFeesJson) {*requestWithFeesJson = outReq;}
    if (paymentMethod) {*paymentMethod = outPayMethod;}
    return err;
}

- (NSError *)parseResponseWithFees:(NSString *)responseJson
                     paymentMethod:(NSString *)paymentMethod
                          utxoJson:(NSString **)utxoJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outUtxo = nil;

    [IndyPayment parseResponseWithFees:responseJson
                         paymentMethod:paymentMethod
                            completion:^(NSError *error, NSString *utxo) {
                                err = error;
                                outUtxo = utxo;
                                [completionExpectation fulfill];
                            }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (utxoJson) {*utxoJson = outUtxo;}

    return err;
}

- (NSError *)buildGetUtxoRequest:(IndyHandle)walletHandle
                    submitterDid:(NSString *)submitterDid
                  paymentAddress:(NSString *)paymentAddress
                  getUtxoTxnJson:(NSString **)getUtxoTxnJson
                   paymentMethod:(NSString **)paymentMethod {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outReq = nil;
    __block NSString *outPayMethod = nil;

    [IndyPayment buildGetUtxoRequest:walletHandle
                        submitterDid:submitterDid
                      paymentAddress:paymentAddress
                          completion:^(NSError *error, NSString *req, NSString *method) {
                              err = error;
                              outReq = req;
                              outPayMethod = method;
                              [completionExpectation fulfill];
                          }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (getUtxoTxnJson) {*getUtxoTxnJson = outReq;}
    if (paymentMethod) {*paymentMethod = outPayMethod;}
    return err;
}

- (NSError *)parseGetUtxoResponse:(NSString *)responseJson
                    paymentMethod:(NSString *)paymentMethod
                         utxoJson:(NSString **)utxoJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outUtxo = nil;

    [IndyPayment parseGetUtxoResponse:responseJson
                        paymentMethod:paymentMethod
                           completion:^(NSError *error, NSString *utxo) {
                               err = error;
                               outUtxo = utxo;
                               [completionExpectation fulfill];
                           }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (utxoJson) {*utxoJson = outUtxo;}

    return err;
}

- (NSError *)buildPaymentRequest:(IndyHandle)walletHandle
                    submitterDid:(NSString *)submitterDid
                      inputsJson:(NSString *)inputsJson
                     outputsJson:(NSString *)outputsJson
                  paymentReqJson:(NSString **)paymentReqJson
                   paymentMethod:(NSString **)paymentMethod {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outReq = nil;
    __block NSString *outPayMethod = nil;

    [IndyPayment buildPaymentRequest:walletHandle
                        submitterDid:submitterDid
                          inputsJson:inputsJson
                         outputsJson:outputsJson
                          completion:^(NSError *error, NSString *req, NSString *method) {
                              err = error;
                              outReq = req;
                              outPayMethod = method;
                              [completionExpectation fulfill];
                          }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (paymentReqJson) {*paymentReqJson = outReq;}
    if (paymentMethod) {*paymentMethod = outPayMethod;}
    return err;
}

- (NSError *)parsePaymentResponse:(NSString *)responseJson
                    paymentMethod:(NSString *)paymentMethod
                         utxoJson:(NSString **)utxoJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outUtxo = nil;

    [IndyPayment parsePaymentResponse:responseJson
                        paymentMethod:paymentMethod
                           completion:^(NSError *error, NSString *utxo) {
                               err = error;
                               outUtxo = utxo;
                               [completionExpectation fulfill];
                           }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (utxoJson) {*utxoJson = outUtxo;}

    return err;
}

- (NSError *)buildMintRequest:(IndyHandle)walletHandle
                 submitterDid:(NSString *)submitterDid
                  outputsJson:(NSString *)outputsJson
                  mintReqJson:(NSString **)mintReqJson
                paymentMethod:(NSString **)paymentMethod {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outReq = nil;
    __block NSString *outPayMethod = nil;

    [IndyPayment buildMintRequest:walletHandle
                     submitterDid:submitterDid
                      outputsJson:outputsJson
                       completion:
                               ^(NSError *error, NSString *req, NSString *method) {
                                   err = error;
                                   outReq = req;
                                   outPayMethod = method;
                                   [completionExpectation fulfill];
                               }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (mintReqJson) {*mintReqJson = outReq;}
    if (paymentMethod) {*paymentMethod = outPayMethod;}
    return err;
}

- (NSError *)buildSetTxnFeesRequest:(IndyHandle)walletHandle
                       submitterDid:(NSString *)submitterDid
                      paymentMethod:(NSString *)paymentMethod
                           feesJson:(NSString *)feesJson
                  setTxnFeesReqJson:(NSString **)setTxnFeesReqJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outUtxo = nil;

    [IndyPayment buildSetTxnFeesRequest:walletHandle
                           submitterDid:submitterDid
                          paymentMethod:paymentMethod
                               feesJson:feesJson
                             completion:^(NSError *error, NSString *utxo) {
                                 err = error;
                                 outUtxo = utxo;
                                 [completionExpectation fulfill];
                             }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (setTxnFeesReqJson) {*setTxnFeesReqJson = outUtxo;}

    return err;
}

- (NSError *)buildGetTxnFeesRequest:(IndyHandle)walletHandle
                       submitterDid:(NSString *)submitterDid
                      paymentMethod:(NSString *)paymentMethod
                  getTxnFeesReqJson:(NSString **)getTxnFeesReqJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outUtxo = nil;

    [IndyPayment buildGetTxnFeesRequest:walletHandle
                           submitterDid:submitterDid
                          paymentMethod:paymentMethod
                             completion:^(NSError *error, NSString *utxo) {
                                 err = error;
                                 outUtxo = utxo;
                                 [completionExpectation fulfill];
                             }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (getTxnFeesReqJson) {*getTxnFeesReqJson = outUtxo;}

    return err;
}

- (NSError *)parseGetTxnFeesResponse:(NSString *)responseJson
                       paymentMethod:(NSString *)paymentMethod
                            feesJson:(NSString **)feesJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outFees = nil;

    [IndyPayment parseGetTxnFeesResponse:responseJson
                           paymentMethod:paymentMethod
                              completion:^(NSError *error, NSString *fees) {
                                  err = error;
                                  outFees = fees;
                                  [completionExpectation fulfill];
                              }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (feesJson) {*feesJson = outFees;}

    return err;
}

@end

