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
 sign key, and sends signed request message to validator pool.
 
 @param requestJSON Request data json.
 @param submitterDid Id of Identity stored in secured Wallet.
 @param poolHandle Pool handle (created by IndyPool::openPoolLedgerWithName).
 @param walletHandle Wallet handle (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)signAndSubmitRequest:(NSString *)requestJSON
                submitterDID:(NSString *)submitterDid
                  poolHandle:(IndyHandle)poolHandle
                walletHandle:(IndyHandle)walletHandle
                  completion:(void (^)(NSError *error, NSString *requestResultJSON))completion;

/**
 Signs request message.

 Adds submitter information to passed request json, signs it with submitter
 sign key.
 
 @param requestJson Request data json.
 @param submitterDid Id of Identity stored in secured Wallet.
 @param walletHandle Wallet handle (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter. Returns signed request json.
 */
+ (void)signRequest:(NSString *)requestJson
       submitterDid:(NSString *)submitterDid
       walletHandle:(IndyHandle)walletHandle
         completion:(void (^)(NSError *error, NSString *requestResultJSON))completion;

/**
 Publishes request message to validator pool (no signing, unlike sign_and_submit_request).
 
 The request is sent to the validator pool as is. It's assumed that it's already prepared.
 
 @param requestJSON Request data json.
 @param poolHandle pool handle (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter. Returns requestResultJSON.
 */
+ (void)submitRequest:(NSString *)requestJSON
           poolHandle:(IndyHandle)poolHandle
           completion:(void (^)(NSError *error, NSString *requestResultJSON))completion;

// MARK: - Nym request

/**
 Builds a NYM request.

 @param submitterDid Id of Identity stored in secured Wallet.
 @param targetDid Id of Identity stored in secured Wallet.
 @param verkey Verification key
 @param alias Alias
 @param role Role of a user NYM record
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildNymRequestWithSubmitterDid:(NSString *)submitterDid
                              targetDID:(NSString *)targetDid
                                 verkey:(NSString *)verkey
                                  alias:(NSString *)alias
                                   role:(NSString *)role
                             completion:(void (^)(NSError *error, NSString *requestJSON))completion;

/**
 Builds a GET_NYM request.
 
 @param submitterDid Id of Identity stored in secured Wallet.
 @param targetDid Id of Identity stored in secured Wallet.
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildGetNymRequestWithSubmitterDid:(NSString *)submitterDid
                                 targetDID:(NSString *)targetDid
                                completion:(void (^)(NSError *error, NSString *requestJSON))completion;

// MARK: - Attribute request

/**
 Builds an ATTRIB request.

 @param submitterDid Id of Identity stored in secured Wallet.
 @param targetDid Id of Identity stored in secured Wallet.
 @param hash Hash of attribute data
 @param raw represented as json, where key is attribute name and value is it's value
 @param enc Encrypted attribute data
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildAttribRequestWithSubmitterDid:(NSString *)submitterDid
                                 targetDID:(NSString *)targetDid
                                      hash:(NSString *)hash
                                       raw:(NSString *)raw
                                       enc:(NSString *)enc
                                completion:(void (^)(NSError *error, NSString *requestJSON))completion;

/**
 Builds a GET_ATTRIB request.
 
 @param submitterDid Id of Identity stored in secured Wallet.
 @param targetDid Id of Identity stored in secured Wallet.
 @param data Name (attribute name)
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildGetAttribRequestWithSubmitterDid:(NSString *)submitterDid
                                    targetDID:(NSString *)targetDid
                                          raw:(NSString *)raw
                                         hash:(NSString *)hash
                                          enc:(NSString *)enc
                                   completion:(void (^)(NSError *error, NSString *requestJSON))completion;

// MARK: - Schema request

/**
 Builds a SCHEMA request.
 
 @param submitterDid Id of Identity stored in secured Wallet.
 @param data Name, version, type, attr_names (ip, port, keys)
 @param completion Callback that takes command result as parameter. Request result as json.
 */
+ (void)buildSchemaRequestWithSubmitterDid:(NSString *)submitterDid
                                      data:(NSString *)data
                                completion:(void (^)(NSError *error, NSString *requestJSON))completion;

/**
 Builds a GET_SCHEMA request.
 
 @param submitterDid Id of Identity stored in secured Wallet.
 @param dest Id of Identity stored in secured Wallet.
 @param data Name, version
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildGetSchemaRequestWithSubmitterDid:(NSString *)submitterDid
                                         dest:(NSString *)dest
                                         data:(NSString *)data
                                   completion:(void (^)(NSError *error, NSString *requestJSON))completion;

// MARK: - ClaimDefTxn request

/**
 Builds an CLAIM_DEF request.
 
 @param submitterDid Id of Identity stored in secured Wallet.
 @param xref Seq. number of schema
 @param signatureType Signature type (only CL supported now)
 @param data Components of a key in json: N, R, S, Z
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildClaimDefTxnWithSubmitterDid:(NSString *)submitterDid
                                    xref:(NSNumber *)xref
                           signatureType:(NSString *)signatureType
                                    data:(NSString *)data
                              completion:(void (^)(NSError *error, NSString *requestJSON))completion;

/**
 Builds a GET_CLAIM_DEF request.
 
 @param submitterDid Id of Identity stored in secured Wallet.
 @param xref Seq. number of schema
 @param signatureType signature type (only CL supported now)
 @param origin issuer did
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildGetClaimDefTxnWithSubmitterDid:(NSString *)submitterDid
                                       xref:(NSNumber *)xref
                              signatureType:(NSString *)signatureType
                                     origin:(NSString *)origin
                                 completion:(void (^)(NSError *error, NSString *requestJSON))completion;

// MARK: - Ddo request

/**
 Builds a request to get a DDO.

 
 @param submitterDid Id of Identity stored in secured Wallet.
 @param targetDid Id of Identity stored in secured Wallet.
 @param completion Callback that takes command result as parameter. Returns result as json.
 */
+ (void)buildGetDdoRequestWithSubmitterDid:(NSString *)submitterDid
                                 targetDID:(NSString *)targetDid
                                completion:(void (^)(NSError *error, NSString *requestResultJSON))completion;

// MARK: - Node request

/**
 Builds a NODE request.
 
 @param submitterDid Id of Identity stored in secured Wallet.
 @param targetDid Id of Identity stored in secured Wallet.
 @param data id of a target NYM record
 @param completion Callback that takes command result as parameter. Return request result as json.
 */
+ (void)buildNodeRequestWithSubmitterDid:(NSString *)submitterDid
                               targetDid:(NSString *)targetDid
                                    data:(NSString *)data
                              completion:(void (^)(NSError *error, NSString *requestJSON))completion;

// MARK: - Txn request

/**
 Builds a GET_TXN request.

 @param submitterDid Id of Identity stored in secured Wallet.
 @param data seq_no of transaction in ledger
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildGetTxnRequestWithSubmitterDid:(NSString *)submitterDid
                                      data:(NSNumber *)data
                                completion:(void (^)(NSError *error, NSString *requestJSON))completion;

// MARK: - Pool config request

/**
 Builds a POOL_CONFIG request.

 @param submitterDid Id of Identity stored in secured Wallet.
 @param writes
 @param force
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildPoolConfigRequestWithSubmitterDid:(NSString *)submitterDid
                                        writes:(BOOL)writes
                                         force:(BOOL)force
                                    completion:(void (^)(NSError *error, NSString *requestJSON))completion;
// MARK: - Pool upgrade request

/**
 Builds a POOL_UPGRADE request.

 @param submitterDid Id of Identity stored in secured Wallet.
 @param name
 @param version
 @param action
 @param sha256
 @param timeout
 @param schedule
 @param justification
 @param reinstall
 @param force
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildPoolUpgradeRequestWithSubmitterDid:(NSString *)submitterDid
                                           name:(NSString *)name
                                        version:(NSString *)version
                                         action:(NSString *)action
                                         sha256:(NSString *)sha256
                                        timeout:(NSNumber *)timeout
                                       schedule:(NSString *)schedule
                                  justification:(NSString *)justification
                                      reinstall:(BOOL)reinstall
                                          force:(BOOL)force
                                     completion:(void (^)(NSError *error, NSString *requestJSON))completion;

@end
