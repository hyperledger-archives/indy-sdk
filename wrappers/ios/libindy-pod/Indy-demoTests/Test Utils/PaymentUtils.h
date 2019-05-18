#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>

@interface PaymentUtils : XCTestCase

+ (PaymentUtils *)sharedInstance;

// MARK: - Payment Address
- (NSError *)createPaymentAddressForMethod:(NSString *)paymentMethod
                              walletHandle:(IndyHandle)walletHandle
                                    config:(NSString *)config
                            paymentAddress:(NSString **)paymentAddress;

- (NSError *)listPaymentAddresses:(IndyHandle)walletHandle
                 paymentAddresses:(NSString **)paymentAddresses;


// MARK: - Request Fees
- (NSError *)addFeesToRequest:(NSString *)requestJson
                 walletHandle:(IndyHandle)walletHandle
                 submitterDid:(NSString *)submitterDid
                   inputsJson:(NSString *)inputsJson
                  outputsJson:(NSString *)outputsJson
                        extra:(NSString *)extra
          requestWithFeesJson:(NSString **)requestWithFeesJson
                paymentMethod:(NSString **)paymentMethod;

- (NSError *)parseResponseWithFees:(NSString *)responseJson
                     paymentMethod:(NSString *)paymentMethod
                      receiptsJson:(NSString **)receiptsJson;

// MARK: - Get sources request
- (NSError *)buildGetPaymentSourcesRequest:(IndyHandle)walletHandle
                              submitterDid:(NSString *)submitterDid
                            paymentAddress:(NSString *)paymentAddress
                         getSourcesTxnJson:(NSString **)getSourcesTxnJson
                             paymentMethod:(NSString **)paymentMethod;

- (NSError *)parseGetPaymentSourcesResponse:(NSString *)responseJson
                              paymentMethod:(NSString *)paymentMethod
                                sourcesJson:(NSString **)sourcesJson;

// MARK: - Payment request
- (NSError *)buildPaymentRequest:(IndyHandle)walletHandle
                    submitterDid:(NSString *)submitterDid
                      inputsJson:(NSString *)inputsJson
                     outputsJson:(NSString *)outputsJson
                           extra:(NSString *)extra
                  paymentReqJson:(NSString **)paymentReqJson
                   paymentMethod:(NSString **)paymentMethod;

- (NSError *)parsePaymentResponse:(NSString *)responseJson
                    paymentMethod:(NSString *)paymentMethod
                     receiptsJson:(NSString **)receiptsJson;

- (NSError *)preparePaymentExtraWithAcceptanceData:(NSString *)extraJson
                                              text:(NSString *)text
                                           version:(NSString *)version
                                         taaDigest:(NSString *)taaDigest
                                       accMechType:(NSString *)accMechType
                                  timeOfAcceptance:(NSNumber *)timeOfAcceptance
                               extraWithAcceptance:(NSString **)extraWithAcceptance;

// MARK: - Mint request
- (NSError *)buildMintRequest:(IndyHandle)walletHandle
                 submitterDid:(NSString *)submitterDid
                  outputsJson:(NSString *)outputsJson
                        extra:(NSString *)extra
                  mintReqJson:(NSString **)mintReqJson
                paymentMethod:(NSString **)paymentMethod;

// MARK: - Set Fees Request
- (NSError *)buildSetTxnFeesRequest:(IndyHandle)walletHandle
                       submitterDid:(NSString *)submitterDid
                      paymentMethod:(NSString *)paymentMethod
                           feesJson:(NSString *)feesJson
                  setTxnFeesReqJson:(NSString **)setTxnFeesReqJson;

- (NSError *)buildGetTxnFeesRequest:(IndyHandle)walletHandle
                       submitterDid:(NSString *)submitterDid
                      paymentMethod:(NSString *)paymentMethod
                  getTxnFeesReqJson:(NSString **)getTxnFeesReqJson;

- (NSError *)parseGetTxnFeesResponse:(NSString *)responseJson
                       paymentMethod:(NSString *)paymentMethod
                            feesJson:(NSString **)feesJson;

// MARK: - Verify request
- (NSError *)buildVerifyPaymentRequest:(IndyHandle)walletHandle
                          submitterDid:(NSString *)submitterDid
                               receipt:(NSString *)receipt
                         verifyReqJson:(NSString **)verifyReqJson
                         paymentMethod:(NSString **)paymentMethod;

- (NSError *)parseVerifyPaymentResponse:(NSString *)responseJson
                          paymentMethod:(NSString *)paymentMethod
                        receiptInfoJson:(NSString **)receiptInfoJson;
@end

