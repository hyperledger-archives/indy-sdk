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
                        extra:(NSString *)extra
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
                            extra:extra
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
                      receiptsJson:(NSString **)receiptsJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outReceipts = nil;

    [IndyPayment parseResponseWithFees:responseJson
                         paymentMethod:paymentMethod
                            completion:^(NSError *error, NSString *receipts) {
                                err = error;
                                outReceipts = receipts;
                                [completionExpectation fulfill];
                            }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (receiptsJson) {*receiptsJson = outReceipts;}

    return err;
}

- (NSError *)buildGetPaymentSourcesRequest:(IndyHandle)walletHandle
                              submitterDid:(NSString *)submitterDid
                            paymentAddress:(NSString *)paymentAddress
                         getSourcesTxnJson:(NSString **)getSourcesTxnJson
                             paymentMethod:(NSString **)paymentMethod {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outReq = nil;
    __block NSString *outPayMethod = nil;

    [IndyPayment buildGetPaymentSourcesRequest:walletHandle
                                  submitterDid:submitterDid
                                paymentAddress:paymentAddress
                                    completion:^(NSError *error, NSString *req, NSString *method) {
                                        err = error;
                                        outReq = req;
                                        outPayMethod = method;
                                        [completionExpectation fulfill];
                                    }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (getSourcesTxnJson) {*getSourcesTxnJson = outReq;}
    if (paymentMethod) {*paymentMethod = outPayMethod;}
    return err;
}

- (NSError *)parseGetPaymentSourcesResponse:(NSString *)responseJson
                              paymentMethod:(NSString *)paymentMethod
                                sourcesJson:(NSString **)sourcesJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outSources = nil;

    [IndyPayment parseGetPaymentSourcesResponse:responseJson
                                  paymentMethod:paymentMethod
                                     completion:^(NSError *error, NSString *sources) {
                                         err = error;
                                         outSources = sources;
                                         [completionExpectation fulfill];
                                     }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (sourcesJson) {*sourcesJson = outSources;}

    return err;
}

- (NSError *)buildPaymentRequest:(IndyHandle)walletHandle
                    submitterDid:(NSString *)submitterDid
                      inputsJson:(NSString *)inputsJson
                     outputsJson:(NSString *)outputsJson
                           extra:(NSString *)extra
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
                               extra:extra
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
                     receiptsJson:(NSString **)receiptsJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outReceipts = nil;

    [IndyPayment parsePaymentResponse:responseJson
                        paymentMethod:paymentMethod
                           completion:^(NSError *error, NSString *receipts) {
                               err = error;
                               outReceipts = receipts;
                               [completionExpectation fulfill];
                           }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (receiptsJson) {*receiptsJson = outReceipts;}

    return err;
}

- (NSError *)preparePaymentExtraWithAcceptanceData:(NSString *)extraJson
                                              text:(NSString *)text
                                           version:(NSString *)version
                                         taaDigest:(NSString *)taaDigest
                                       accMechType:(NSString *)accMechType
                                  timeOfAcceptance:(NSNumber *)timeOfAcceptance
                               extraWithAcceptance:(NSString **)extraWithAcceptance {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outJson = nil;

    [IndyPayment preparePaymentExtraWithAcceptanceData:extraJson
                                                  text:text
                                               version:version
                                             taaDigest:taaDigest
                                           accMechType:accMechType
                                      timeOfAcceptance:timeOfAcceptance
                                            completion:^(NSError *error, NSString *json) {
                                                err = error;
                                                outJson = json;
                                                [completionExpectation fulfill];
                                            }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (extraWithAcceptance) {*extraWithAcceptance = outJson;}

    return err;
}

- (NSError *)buildMintRequest:(IndyHandle)walletHandle
                 submitterDid:(NSString *)submitterDid
                  outputsJson:(NSString *)outputsJson
                        extra:(NSString *)extra
                  mintReqJson:(NSString **)mintReqJson
                paymentMethod:(NSString **)paymentMethod {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outReq = nil;
    __block NSString *outPayMethod = nil;

    [IndyPayment buildMintRequest:walletHandle
                     submitterDid:submitterDid
                      outputsJson:outputsJson
                            extra:extra
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
    __block NSString *outSources = nil;

    [IndyPayment buildSetTxnFeesRequest:walletHandle
                           submitterDid:submitterDid
                          paymentMethod:paymentMethod
                               feesJson:feesJson
                             completion:^(NSError *error, NSString *sources) {
                                 err = error;
                                 outSources = sources;
                                 [completionExpectation fulfill];
                             }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (setTxnFeesReqJson) {*setTxnFeesReqJson = outSources;}

    return err;
}

- (NSError *)buildGetTxnFeesRequest:(IndyHandle)walletHandle
                       submitterDid:(NSString *)submitterDid
                      paymentMethod:(NSString *)paymentMethod
                  getTxnFeesReqJson:(NSString **)getTxnFeesReqJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outU = nil;

    [IndyPayment buildGetTxnFeesRequest:walletHandle
                           submitterDid:submitterDid
                          paymentMethod:paymentMethod
                             completion:^(NSError *error, NSString *temp) {
                                 err = error;
                                 outU = temp;
                                 [completionExpectation fulfill];
                             }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (getTxnFeesReqJson) {*getTxnFeesReqJson = outU;}

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

- (NSError *)buildVerifyPaymentRequest:(IndyHandle)walletHandle
                          submitterDid:(NSString *)submitterDid
                               receipt:(NSString *)receipt
                         verifyReqJson:(NSString **)verifyReqJson
                         paymentMethod:(NSString **)paymentMethod {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outReq = nil;
    __block NSString *outPayMethod = nil;

    [IndyPayment buildVerifyPaymentRequest:walletHandle
                              submitterDid:submitterDid
                                   receipt:receipt
                                completion:^(NSError *error, NSString *req, NSString *method) {
                                    err = error;
                                    outReq = req;
                                    outPayMethod = method;
                                    [completionExpectation fulfill];
                                }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (verifyReqJson) {*verifyReqJson = outReq;}
    if (paymentMethod) {*paymentMethod = outPayMethod;}
    return err;
}

- (NSError *)parseVerifyPaymentResponse:(NSString *)responseJson
                          paymentMethod:(NSString *)paymentMethod
                        receiptInfoJson:(NSString **)receiptInfoJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outReceipts = nil;

    [IndyPayment parseVerifyPaymentResponse:responseJson
                              paymentMethod:paymentMethod
                                 completion:^(NSError *error, NSString *receipts) {
                                     err = error;
                                     outReceipts = receipts;
                                     [completionExpectation fulfill];
                                 }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (receiptInfoJson) {*receiptInfoJson = outReceipts;}

    return err;
}

@end

