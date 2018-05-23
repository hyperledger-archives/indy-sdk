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
          requestWithFeesJson:(NSString **)requestWithFeesJson
                paymentMethod:(NSString **)paymentMethod;

- (NSError *)parseResponseWithFees:(NSString *)responseJson
                     paymentMethod:(NSString *)paymentMethod
                          utxoJson:(NSString **)utxoJson;

// MARK: - Get UTXO request
- (NSError *)buildGetUtxoRequest:(IndyHandle)walletHandle
                    submitterDid:(NSString *)submitterDid
                  paymentAddress:(NSString *)paymentAddress
                  getUtxoTxnJson:(NSString **)getUtxoTxnJson
                   paymentMethod:(NSString **)paymentMethod;

- (NSError *)parseGetUtxoResponse:(NSString *)responseJson
                    paymentMethod:(NSString *)paymentMethod
                         utxoJson:(NSString **)utxoJson;

// MARK: - Payment request
- (NSError *)buildPaymentRequest:(IndyHandle)walletHandle
                    submitterDid:(NSString *)submitterDid
                      inputsJson:(NSString *)inputsJson
                     outputsJson:(NSString *)outputsJson
                  paymentReqJson:(NSString **)paymentReqJson
                   paymentMethod:(NSString **)paymentMethod;

- (NSError *)parsePaymentResponse:(NSString *)responseJson
                    paymentMethod:(NSString *)paymentMethod
                         utxoJson:(NSString **)utxoJson;

// MARK: - Mint request
- (NSError *)buildMintRequest:(IndyHandle)walletHandle
                 submitterDid:(NSString *)submitterDid
                  outputsJson:(NSString *)outputsJson
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
@end

