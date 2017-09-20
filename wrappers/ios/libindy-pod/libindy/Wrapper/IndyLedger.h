//
//  IndyLedger.h
//  libindy
//


#import <Foundation/Foundation.h>
#import "IndyTypes.h"

@interface IndyLedger : NSObject

/**
 Signs and submits request message to validator pool.
 
 Adds submitter information to passed request json, signs it with submitter
 sign key (see wallet_sign), and sends signed request message
 to validator pool (see write_request).
 
 @param walletHandle Wallet handle (created by IndyWallet::openWalletWithName).
 @param poolHandle Pool handle (created by IndyPool::openPoolLedgerWithName).
 @param submitterDid Id of Identity stored in secured Wallet.
 @param requestJSON Request data json.
 @param handler Callback that takes command result as parameter. Returns request result as json.

 @return ErrorCode
 */
+ (NSError *)signAndSubmitRequestWithWalletHandle:(IndyHandle)walletHandle
                                       poolHandle:(IndyHandle)poolHandle
                                     submitterDID:(NSString *)submitterDid
                                      requestJSON:(NSString *)requestJSON
                                       completion:(void (^)(NSError *error, NSString *requestResultJSON)) handler;

/**
 Signs request message.

 Adds submitter information to passed request json, signs it with submitter
 sign key (see wallet_sign).
 
 @param walletHandle Wallet handle (created by IndyWallet::openWalletWithName).
 @param submitterDid Id of Identity stored in secured Wallet.
 @param requestJson Request data json.
 @param handler Callback that takes command result as parameter. Returns signed request json.

 @return Error Code
 */
+ (NSError *)signRequestWithWalletHandle:(IndyHandle)walletHandle
                            submitterDid:(NSString *)submitterDid
                             requestJson:(NSString *)requestJson
                              completion:(void (^)(NSError *error, NSString *requestResultJSON)) handler;

/**
 Publishes request message to validator pool (no signing, unlike sign_and_submit_request).
 
 The request is sent to the validator pool as is. It's assumed that it's already prepared.
 
 @param poolHandle pool handle (created by IndyWallet::openWalletWithName).
 @param requestJSON Request data json.
 @param handler Callback that takes command result as parameter. Returns requestResultJSON.
 
 @return Error Code
 */
+ (NSError *)submitRequestWithPoolHandle:(IndyHandle)poolHandle
                             requestJSON:(NSString *)requestJSON
                              completion:(void (^)(NSError *error, NSString *requestResultJSON)) handler;

// MARK: - Nym request

/**
 Builds a NYM request.

 @param submitterDid Id of Identity stored in secured Wallet.
 @param targetDid Id of Identity stored in secured Wallet.
 @param verkey Verification key
 @param alias Alias
 @param role Role of a user NYM record
 @param handler Callback that takes command result as parameter. Returns request result as json.
 
 @return Error Code
 */
+ (NSError *)buildNymRequestWithSubmitterDid:(NSString *)submitterDid
                                   targetDID:(NSString *)targetDid
                                      verkey:(NSString *)verkey
                                       alias:(NSString *)alias
                                        role:(NSString *)role
                                  completion:(void (^)(NSError *error, NSString *requestJSON)) handler;

/**
 Builds a GET_NYM request.
 
 @param submitterDid Id of Identity stored in secured Wallet.
 @param targetDid Id of Identity stored in secured Wallet.
 @param handler Callback that takes command result as parameter. Returns request result as json.

 @return Error Code
 */
+ (NSError *)buildGetNymRequestWithSubmitterDid:(NSString *)submitterDid
                                      targetDID:(NSString *)targetDid
                                     completion:(void (^)(NSError *error, NSString *requestJSON)) handler;

// MARK: - Attribute request

/**
 Builds an ATTRIB request.

 @param submitterDid Id of Identity stored in secured Wallet.
 @param targetDid Id of Identity stored in secured Wallet.
 @param hash Hash of attribute data
 @param raw represented as json, where key is attribute name and value is it's value
 @param enc Encrypted attribute data
 @param handler Callback that takes command result as parameter. Returns request result as json.

 @return Error Code
 */
+ (NSError *)buildAttribRequestWithSubmitterDid:(NSString *)submitterDid
                                      targetDID:(NSString *)targetDid
                                           hash:(NSString *)hash
                                            raw:(NSString *)raw
                                            enc:(NSString *)enc
                                     completion:(void (^)(NSError *error, NSString *requestJSON)) handler;

/**
 Builds a GET_ATTRIB request.
 
 @param submitterDid Id of Identity stored in secured Wallet.
 @param targetDid Id of Identity stored in secured Wallet.
 @param data Name (attribute name)
 @param handler Callback that takes command result as parameter. Returns request result as json.

 @return Error Code
 */
+ (NSError *)buildGetAttribRequestWithSubmitterDid:(NSString *)submitterDid
                                         targetDID:(NSString *)targetDid
                                              data:(NSString *)data
                                        completion:(void (^)(NSError *error, NSString *requestJSON)) handler;

// MARK: - Schema request

/**
 Builds a SCHEMA request.
 
 @param submitterDid Id of Identity stored in secured Wallet.
 @param data Name, version, type, attr_names (ip, port, keys)
 @param handler Callback that takes command result as parameter. Request result as json.
 
 @return Error Code
 */
+ (NSError *)buildSchemaRequestWithSubmitterDid:(NSString *)submitterDid
                                           data:(NSString *)data
                                     completion:(void (^)(NSError *error, NSString *requestJSON)) handler;

/**
 Builds a GET_SCHEMA request.
 
 @param submitterDid Id of Identity stored in secured Wallet.
 @param dest Id of Identity stored in secured Wallet.
 @param data Name, version
 @param handler Callback that takes command result as parameter. Returns request result as json.
 
 @return Error Code
 */
+ (NSError *)buildGetSchemaRequestWithSubmitterDid:(NSString *)submitterDid
                                              dest:(NSString *)dest
                                              data:(NSString *)data
                                        completion:(void (^)(NSError *error, NSString *requestJSON)) handler;

// MARK: - ClaimDefTxn request

/**
 Builds an CLAIM_DEF request.
 
 @param submitterDid Id of Identity stored in secured Wallet.
 @param xref Seq. number of schema
 @param signatureType Signature type (only CL supported now)
 @param data Components of a key in json: N, R, S, Z
 @param handler Callback that takes command result as parameter. Returns request result as json.

 @return Error Code
 */
+ (NSError *)buildClaimDefTxnWithSubmitterDid:(NSString *)submitterDid
                                         xref:(NSString *)xref
                                signatureType:(NSString *)signatureType
                                         data:(NSString *)data
                                   completion:(void (^)(NSError *error, NSString *requestJSON)) handler;

/**
 Builds a GET_CLAIM_DEF request.
 
 @param submitterDid Id of Identity stored in secured Wallet.
 @param xref Seq. number of schema
 @param signatureType signature type (only CL supported now)
 @param origin issuer did
 @param handler Callback that takes command result as parameter. Returns request result as json.

 @return Error Code
 */
+ (NSError *)buildGetClaimDefTxnWithSubmitterDid:(NSString *)submitterDid
                                            xref:(NSString *)xref
                                   signatureType:(NSString *)signatureType
                                          origin:(NSString *)origin
                                      completion:(void (^)(NSError *error, NSString *requestJSON)) handler;

// MARK: - Ddo request

/**
 Builds a request to get a DDO.

 
 @param submitterDid Id of Identity stored in secured Wallet.
 @param targetDid Id of Identity stored in secured Wallet.
 @param handler Callback that takes command result as parameter. Returns result as json.

 @return Error Code
 */
+ (NSError *)buildGetDdoRequestWithSubmitterDid:(NSString *)submitterDid
                                      targetDID:(NSString *)targetDid
                                     completion:(void (^)(NSError *error, NSString *requestResultJSON)) handler;

// MARK: - Node request

/**
 Builds a NODE request.
 
 @param submitterDid Id of Identity stored in secured Wallet.
 @param targetDid Id of Identity stored in secured Wallet.
 @param data id of a target NYM record
 @param handler Callback that takes command result as parameter. Return request result as json.

 @return Error Code
 */
+ (NSError *)buildNodeRequestWithSubmitterDid:(NSString *)submitterDid
                                    targetDid:(NSString *)targetDid
                                         data:(NSString *)data
                                   completion:(void (^)(NSError *error, NSString *requestJSON)) handler;

// MARK: - Txn request

/**
 Builds a GET_TXN request.

 @param submitterDid Id of Identity stored in secured Wallet.
 @param data seq_no of transaction in ledger
 @param handler Callback that takes command result as parameter. Returns request result as json.
 */
+ (NSError *)buildGetTxnRequestWithSubmitterDid:(NSString *)submitterDid
                                           data:(NSNumber *)data
                                     completion:(void (^)(NSError *error, NSString *requestJSON)) handler;

@end
