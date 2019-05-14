//
// Created by Evernym on 5/14/18.
// Copyright (c) 2018 Hyperledger. All rights reserved.
//

#import <Foundation/Foundation.h>
#import "IndyTypes.h"


@interface IndyPayment : NSObject

/**
 Create the payment address for specified payment method.

 This method generates private part of payment address
 and stores it in a secure place. Ideally it should be
 secret in libindy wallet (see crypto module).

 @param paymentMethod Payment method to use (for example, 'sov')
 @param walletHandle Wallet handle (created by IndyWallet::openWalletWithName).
 @param config payment address config as json:
   {
     seed: <str>, // allows deterministic creation of payment address
   }
 @param completion Callback that takes command result as parameter. Returns payment address.
 */
+ (void)createPaymentAddressForMethod:(NSString *)paymentMethod
                         walletHandle:(IndyHandle)walletHandle
                               config:(NSString *)config
                           completion:(void (^)(NSError *error, NSString *paymentAddress))completion;

/**a
 Lists all payment addresses that are stored in the wallet

 @param walletHandle Wallet handle (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter. Returns list of payment addresses json.
 */
+ (void)listPaymentAddresses:(IndyHandle)walletHandle
                  completion:(void (^)(NSError *error, NSString *paymentAddresses))completion;

/**
 Modifies Indy request by adding information how to pay fees for this transaction
 according to this payment method.

 This method consumes set of inputs and outputs. The difference between inputs balance
 and outputs balance is the fee for this transaction.

 Not that this method also produces correct fee signatures.

 Format of inputs is specific for payment method. Usually it should reference payment transaction
 with at least one output that corresponds to payment address that user owns.

 @param requestJson Request data json.
 @param submitterDid (Optional) DID of request sender
 @param walletHandle Wallet handle (created by IndyWallet::openWalletWithName).
 @param inputsJson The list of payment sources as json array:
   ["source1", ...]
     - each input should reference paymentAddress
     - this param will be used to determine payment_method
 @param outputsJson The list of outputs as json array:
   [{
     recipient: <str>, // payment address of recipient
     amount: <int>, // amount
   }]
 @param extra Optional information for payment operation
 @param completion Callback that takes command result as parameter. Returns addRequestFeesRequest json.
 */
+ (void)addFeesToRequest:(NSString *)requestJson
            walletHandle:(IndyHandle)walletHandle
            submitterDid:(NSString *)submitterDid
              inputsJson:(NSString *)inputsJson
             outputsJson:(NSString *)outputsJson
                   extra:(NSString *)extra
              completion:(void (^)(NSError *error, NSString *requestWithFeesJson, NSString *paymentMethod))completion;

/**
 Parses response for Indy request with fees.

 @param responseJson Response for Indy request with fees
 @param paymentMethod
 @param completion Callback that takes command result as parameter. 
 Returns receiptsJson - parsed (payment method and node version agnostic) receipts info as json:
   [{
      receipt: <str>, // receipt that can be used for payment referencing and verification
      recipient: <str>, //payment address of recipient
      amount: <int>, // amount
      extra: <str>, // optional data from payment transaction
   }]
 */
+ (void)parseResponseWithFees:(NSString *)responseJson
                paymentMethod:(NSString *)paymentMethod
                   completion:(void (^)(NSError *error, NSString *receiptsJson))completion;

/**
 Builds Indy request for getting sources list for payment address
 according to this payment method.

 @param requestJson Request data json.
 @param submitterDid (Optional) DID of request sender
 @param walletHandle Wallet handle (created by IndyWallet::openWalletWithName).
 @param paymentAddress Target payment address
 @param completion Callback that takes command result as parameter. 
 Returns 
    getSourcesTxnJson - Indy request for getting sources list for payment address
    paymentMethod - used payment method
 */
+ (void)buildGetPaymentSourcesRequest:(IndyHandle)walletHandle
                         submitterDid:(NSString *)submitterDid
                       paymentAddress:(NSString *)paymentAddress
                           completion:(void (^)(NSError *error, NSString *getSourcesTxnJson, NSString *paymentMethod))completion;


/**
 Parses response for Indy request for getting sources list.

 @param responseJson response for Indy request for getting sources list
 @param paymentMethod
 @param completion Callback that takes command result as parameter.
 Returns sourcesJson - parsed (payment method and node version agnostic) sources info as json:
   [{
      source: <str>, // source input
      paymentAddress: <str>, //payment address for this source
      amount: <int>, // amount
      extra: <str>, // optional data from payment transaction
   }]
 */
+ (void)parseGetPaymentSourcesResponse:(NSString *)responseJson
                         paymentMethod:(NSString *)paymentMethod
                            completion:(void (^)(NSError *error, NSString *sourcesJson))completion;


/**
 Builds Indy request for doing payment
 according to this payment method.

 This method consumes set of inputs and outputs.

 Format of inputs is specific for payment method. Usually it should reference payment transaction
 with at least one output that corresponds to payment address that user owns.

 @param requestJson Request data json.
 @param submitterDid (Optional) DID of request sender
 @param walletHandle Wallet handle (created by IndyWallet::openWalletWithName).
 @param inputsJson The list of payment sources as json array:
   ["source1", ...]
   Note that each source should reference payment address
 @param outputsJson The list of outputs as json array:
   [{
     recipient: <str>, // payment address of recipient
     amount: <int>, // amount
   }]
 @param extra Optional information for payment operation
 @param completion Callback that takes command result as parameter. 
 Returns 
    paymentRequest - Indy request for doing payment.
    paymentMethod - used payment method
 */
+ (void)buildPaymentRequest:(IndyHandle)walletHandle
               submitterDid:(NSString *)submitterDid
                 inputsJson:(NSString *)inputsJson
                outputsJson:(NSString *)outputsJson
                      extra:(NSString *)extra
                 completion:(void (^)(NSError *error, NSString *paymentReqJson, NSString *paymentMethod))completion;

/**
 Parses response for Indy request for payment txn.

 @param responseJson response for Indy request for payment txn
 @param paymentMethod
 @param completion Callback that takes command result as parameter.
 Returns receiptsJson : parsed (payment method and node version agnostic) receipts info as json:
   [{
      receipt: <str>, // receipt that can be used for payment referencing and verification
      recipient: <str>, // payment address of recipient
      amount: <int>, // amount
      extra: <str>, // optional data from payment transaction
   }]
 */
+ (void)parsePaymentResponse:(NSString *)responseJson
               paymentMethod:(NSString *)paymentMethod
                  completion:(void (^)(NSError *error, NSString *receiptsJson))completion;

/**
 Append payment extra JSON with TAA acceptance data

 EXPERIMENTAL

 This function may calculate hash by itself or consume it as a parameter.
 If all text, version and taaDigest parameters are specified, a check integrity of them will be done.

 @param extraJson original extra json.
 @param text (Optional) raw data about TAA from ledger.
 @param version (Optional) version of TAA from ledger.
     text and version should be passed together.
     text and version are required if taaDigest parameter is omitted.
 @param taaDigest (Optional) hash on text and version. This parameter is required if text and version parameters are omitted.
 @param accMechType mechanism how user has accepted the TAA
 @param timeOfAcceptance UTC timestamp when user has accepted the TAA

 Returns Updated request result as json.
 */
+ (void)preparePaymentExtraWithAcceptanceData:(NSString *)extraJson
                                         text:(NSString *)text
                                      version:(NSString *)version
                                    taaDigest:(NSString *)taaDigest
                                  accMechType:(NSString *)accMechType
                             timeOfAcceptance:(NSNumber *)timeOfAcceptance
                                   completion:(void (^)(NSError *error, NSString *extraWithAcceptance))completion;

/**
 Builds Indy request for doing minting according to this payment method.

 @param requestJson Request data json.
 @param submitterDid (Optional) DID of request sender
 @param walletHandle Wallet handle (created by IndyWallet::openWalletWithName).
 @param outputsJson The list of outputs as json array:
   [{
     recipient: <str>, // payment address of recipient
     amount: <int>, // amount
   }]
 @param extra Optional information for mint operation
 @param completion Callback that takes command result as parameter.
 Returns
    MintRequest - Indy request for doing minting
    paymentMethod - used payment method
 */
+ (void)buildMintRequest:(IndyHandle)walletHandle
            submitterDid:(NSString *)submitterDid
             outputsJson:(NSString *)outputsJson
                   extra:(NSString *)extra
              completion:(void (^)(NSError *error, NSString *mintReqJson, NSString *paymentMethod))completion;

/**
 Builds Indy request for setting fees for transactions in the ledger

 @param requestJson Request data json.
 @param submitterDid (Optional) DID of request sender
 @param walletHandle Wallet handle (created by IndyWallet::openWalletWithName).
 @param paymentMethod
 @param feesJson {
   txnType1: amount1,
   txnType2: amount2,
   .................
   txnTypeN: amountN,
 }
 @param completion Callback that takes command result as parameter. Returns setTxnFeesRequest json.
 */
+ (void)buildSetTxnFeesRequest:(IndyHandle)walletHandle
                  submitterDid:(NSString *)submitterDid
                 paymentMethod:(NSString *)paymentMethod
                      feesJson:(NSString *)feesJson
                    completion:(void (^)(NSError *error, NSString *setTxnFeesReqJson))completion;

/**
 Builds Indy request for getting fees for transactions in the ledger

 @param requestJson Request data json.
 @param submitterDid (Optional) DID of request sender
 @param walletHandle Wallet handle (created by IndyWallet::openWalletWithName).
 @param paymentMethod
 @param completion Callback that takes command result as parameter. Returns getTxnFeesRequest json.
 */
+ (void)buildGetTxnFeesRequest:(IndyHandle)walletHandle
                  submitterDid:(NSString *)submitterDid
                 paymentMethod:(NSString *)paymentMethod
                    completion:(void (^)(NSError *error, NSString *getTxnFeesReqJson))completion;

/**
 Parses response for Indy request for getting fees

 @param responseJson response for Indy request for getting fees
 @param paymentMethod
 @param completion Callback that takes command result as parameter.
 Returns feesJson {
   txnType1: amount1,
   txnType2: amount2,
   .................
   txnTypeN: amountN,
 }
 */
+ (void)parseGetTxnFeesResponse:responseJson
                  paymentMethod:(NSString *)paymentMethod
                     completion:(void (^)(NSError *error, NSString *feesJson))completion;


/**
 Builds Indy request for information to verify the payment receipt

 @param submitterDid (Optional) DID of request sender
 @param walletHandle Wallet handle (created by IndyWallet::openWalletWithName).
 @param receipt: Payment receipt to verify
 @param completion Callback that takes command result as parameter. 
 Returns 
    verifyReqJson - Indy request for verification receipt
    paymentMethod - used payment method
 */
+ (void)buildVerifyPaymentRequest:(IndyHandle)walletHandle
                     submitterDid:(NSString *)submitterDid
                          receipt:(NSString *)receipt
                       completion:(void (^)(NSError *error, NSString *verifyReqJson, NSString *paymentMethod))completion;

/**
 Parses Indy response with information to verify receipt

 @param responseJson response for Indy request for payment txn
 @param paymentMethod
 @param completion Callback that takes command result as parameter.
 Returns receiptsJson : parsed (payment method and node version agnostic) receipt info as json:
   {
     sources: [<str>, ]
     receipts: [ {
         recipient: <str>, // payment address of recipient
         receipt: <str>, // receipt that can be used for payment referencing and verification
         amount: <int>, // amount
     } ],
     extra: <str>, //optional data
 }
 */
+ (void)parseVerifyPaymentResponse:(NSString *)responseJson
                     paymentMethod:(NSString *)paymentMethod
                        completion:(void (^)(NSError *error, NSString *txnJson))completion;

@end