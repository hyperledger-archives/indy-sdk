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
 according to selected payment method.

 Payment selection is performed by looking to o

 This method consumes set of UTXO inputs and outputs. The difference between inputs balance
 and outputs balance is the fee for this transaction.

 Not that this method also produces correct fee signatures.

 Format of inputs is specific for payment method. Usually it should reference payment transaction
 with at least one output that corresponds to payment address that user owns.

 @param requestJson Request data json.
 @param submitterDid Id of Identity stored in secured Wallet.
 @param walletHandle Wallet handle (created by IndyWallet::openWalletWithName).
 @param inputsJson The list of UTXO inputs as json array:
   ["input1", ...]
   Notes:
     - each input should reference paymentAddress
     - this param will be used to determine payment_method
 @param outputsJson The list of UTXO outputs as json array:
   [{
     paymentAddress: <str>, // payment address used as output
     amount: <int>, // amount of tokens to transfer to this payment address
     extra: <str>, // optional data
   }]
 @param completion Callback that takes command result as parameter. Returns addRequestFeesRequest json.
 */
+ (void)addFeesToRequest:(NSString *)requestJson
            walletHandle:(IndyHandle)walletHandle
            submitterDid:(NSString *)submitterDid
              inputsJson:(NSString *)inputsJson
             outputsJson:(NSString *)outputsJson
              completion:(void (^)(NSError *error, NSString *requestWithFeesJson, NSString *paymentMethod))completion;

/**
 Parses response for Indy request with fees.

 @param responseJson Response for Indy request with fees
 @param paymentMethod
 @param completion Callback that takes command result as parameter. Returns requestResultJSON.
 */
+ (void)parseResponseWithFees:(NSString *)responseJson
                paymentMethod:(NSString *)paymentMethod
                   completion:(void (^)(NSError *error, NSString *utxoJson))completion;

/**
 Builds Indy request for getting UTXO list for payment address
 according to this payment method.payment transaction
 with at least one output that corresponds to payment address that user owns.

 @param requestJson Request data json.
 @param submitterDid Id of Identity stored in secured Wallet.
 @param walletHandle Wallet handle (created by IndyWallet::openWalletWithName).
 @param paymentAddress Target payment address
 @param completion Callback that takes command result as parameter. Returns getUtxoTxnRequest json.
 */
+ (void)buildGetUtxoRequest:(IndyHandle)walletHandle
               submitterDid:(NSString *)submitterDid
             paymentAddress:(NSString *)paymentAddress
                 completion:(void (^)(NSError *error, NSString *getUtxoTxnJson, NSString *paymentMethod))completion;


/**
 Parses response for Indy request for getting UTXO list.

 @param responseJson response for Indy request for getting UTXO list
 @param paymentMethod
 @param completion Callback that takes command result as parameter.
 Returns utxoJson : parsed (payment method and node version agnostic) utxo info as json:
   [{
      input: <str>, // UTXO input
      amount: <int>, // amount of tokens in this input
      extra: <str>, // optional data from payment transaction
   }]
 */
+ (void)parseGetUtxoResponse:(NSString *)responseJson
               paymentMethod:(NSString *)paymentMethod
                  completion:(void (^)(NSError *error, NSString *utxoJson))completion;


/**
 Builds Indy request for doing tokens payment
 according to this payment method.

 This method consumes set of UTXO inputs and outputs.

 Format of inputs is specific for payment method. Usually it should reference payment transaction
 with at least one output that corresponds to payment address that user owns.

 @param requestJson Request data json.
 @param submitterDid Id of Identity stored in secured Wallet.
 @param walletHandle Wallet handle (created by IndyWallet::openWalletWithName).
 @param inputsJson The list of UTXO inputs as json array:
   ["input1", ...]
   Note that each input should reference paymentAddress
 @param outputsJson The list of UTXO outputs as json array:
   [{
     paymentAddress: <str>, // payment address used as output
     amount: <int>, // amount of tokens to transfer to this payment address
     extra: <str>, // optional data
   }]
 @param completion Callback that takes command result as parameter. Returns paymentRequest json.
 */
+ (void)buildPaymentRequest:(IndyHandle)walletHandle
               submitterDid:(NSString *)submitterDid
                 inputsJson:(NSString *)inputsJson
                outputsJson:(NSString *)outputsJson
                 completion:(void (^)(NSError *error, NSString *paymentReqJson, NSString *paymentMethod))completion;

/**
 Parses response for Indy request for payment txn.

 @param responseJson response for Indy request for payment txn
 @param paymentMethod
 @param completion Callback that takes command result as parameter.
 Returns utxoJson : parsed (payment method and node version agnostic) utxo info as json:
   [{
      input: <str>, // UTXO input
      amount: <int>, // amount of tokens in this input
      extra: <str>, // optional data from payment transaction
   }]
 */
+ (void)parsePaymentResponse:(NSString *)responseJson
               paymentMethod:(NSString *)paymentMethod
                  completion:(void (^)(NSError *error, NSString *utxoJson))completion;

/**
 Builds Indy request for doing tokens minting according to this payment method.

 @param requestJson Request data json.
 @param submitterDid Id of Identity stored in secured Wallet.
 @param walletHandle Wallet handle (created by IndyWallet::openWalletWithName).
 @param outputsJson The list of UTXO outputs as json array:
   [{
     paymentAddress: <str>, // payment address used as output
     amount: <int>, // amount of tokens to transfer to this payment address
     extra: <str>, // optional data
   }]
 @param completion Callback that takes command result as parameter. Returns MintRequest json.
 */
+ (void)buildMintRequest:(IndyHandle)walletHandle
            submitterDid:(NSString *)submitterDid
             outputsJson:(NSString *)outputsJson
              completion:(void (^)(NSError *error, NSString *mintReqJson, NSString *paymentMethod))completion;

/**
 Builds Indy request for setting fees for transactions in the ledger

 @param requestJson Request data json.
 @param submitterDid Id of Identity stored in secured Wallet.
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
 @param submitterDid Id of Identity stored in secured Wallet.
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


@end