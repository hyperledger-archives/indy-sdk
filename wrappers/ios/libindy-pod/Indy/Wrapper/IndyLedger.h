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
 Builds a NYM request. Request to create a new NYM record for a specific user.

 @param submitterDid DID of the submitter stored in secured Wallet.
 @param targetDid Target DID as base58-encoded string for 16 or 32 bit DID value.
 @param verkey Target identity verification key as base58-encoded string.
 @param alias NYM's alias.
 @param role Role of a user NYM record:
                                 null (common USER)
                                 TRUSTEE
                                 STEWARD
                                 TRUST_ANCHOR
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildNymRequestWithSubmitterDid:(NSString *)submitterDid
                              targetDID:(NSString *)targetDid
                                 verkey:(NSString *)verkey
                                  alias:(NSString *)alias
                                   role:(NSString *)role
                             completion:(void (^)(NSError *error, NSString *requestJSON))completion;

/**
 Builds a GET_NYM request. Request to get information about a DID (NYM).
 
 @param submitterDid DID of the read request sender.
 @param targetDid Target DID as base58-encoded string for 16 or 32 bit DID value.
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildGetNymRequestWithSubmitterDid:(NSString *)submitterDid
                                 targetDID:(NSString *)targetDid
                                completion:(void (^)(NSError *error, NSString *requestJSON))completion;

// MARK: - Attribute request

/**
 Builds an ATTRIB request. Request to add attribute to a NYM record.

 @param submitterDid DID of the read request sender.
 @param targetDid Target DID as base58-encoded string for 16 or 32 bit DID value.
 @param raw (Optional) Requested attribute name.
 @param hash (Optional) Requested attribute hash.
 @param enc (Optional) Requested attribute encrypted value.
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildAttribRequestWithSubmitterDid:(NSString *)submitterDid
                                 targetDID:(NSString *)targetDid
                                      hash:(NSString *)hash
                                       raw:(NSString *)raw
                                       enc:(NSString *)enc
                                completion:(void (^)(NSError *error, NSString *requestJSON))completion;

/**
 Builds a GET_ATTRIB request. Request to get information about an Attribute for the specified DID.
 
 @param submitterDid DID of the read request sender.
 @param targetDid Target DID as base58-encoded string for 16 or 32 bit DID value.
 @param raw (Optional) Requested attribute name.
 @param hash (Optional) Requested attribute hash.
 @param enc (Optional) Requested attribute encrypted value.
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
 Builds a SCHEMA request. Request to add Credential's schema.
 
 @param submitterDid DID of the submitter stored in secured Wallet.
 @param data  {
     id: identifier of schema
     attrNames: array of attribute name strings
     name: Schema's name string
     version: Schema's version string
 }
 @param completion Callback that takes command result as parameter. Request result as json.
 */
+ (void)buildSchemaRequestWithSubmitterDid:(NSString *)submitterDid
                                      data:(NSString *)data
                                completion:(void (^)(NSError *error, NSString *requestJSON))completion;

/**
 Builds a GET_SCHEMA request. Request to get Claim's Schema.
 
 @param submitterDid DID of the read request sender.
 @param id Schema ID in ledger
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildGetSchemaRequestWithSubmitterDid:(NSString *)submitterDid
                                           id:(NSString *)id
                                   completion:(void (^)(NSError *error, NSString *requestJSON))completion;

/**
 Parse a GET_SCHEMA response.
 
 @param getSchemaResponse  response json
 @param completion Callback that takes command result as parameter. Schema Id and Schema json.
 */
+ (void)parseGetSchemaResponse:(NSString *)getSchemaResponse
                    completion:(void (^)(NSError *error, NSString *schemaId, NSString *schemaJson))completion;

// MARK: - CredDefRequest request

/**
 Builds an CRED_DEF request. Request to add a Credential Definition (in particular, public key),
 that Issuer creates for a particular Credential Schema.
 
 @param submitterDid DID of the submitter stored in secured Wallet.
 @param data credential definition json
 {
     id: string - identifier of credential definition
     schemaId: string - identifier of stored in ledger schema
     type: string - type of the credential definition. CL is the only supported type now.
     tag: string - allows to distinct between credential definitions for the same issuer and schema
     value: Dictionary with Credential Definition's data: {
         primary: primary credential public key,
         Optional<revocation>: revocation credential public key
     },
     ver: Version of the CredDef json
 }
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildCredDefRequestWithSubmitterDid:(NSString *)submitterDid
                                       data:(NSString *)data
                                 completion:(void (^)(NSError *error, NSString *requestJSON))completion;

/**
 Builds a GET_CRED_DEF request. Request to get a Credential Definition (in particular, public key),
 that Issuer creates for a particular Credential Schema.
 
 @param submitterDid DID of the read request sender.
 @param id Credential Definition ID in ledger
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildGetClaimDefRequestWithSubmitterDid:(NSString *)submitterDid
                                             id:(NSString *)id
                                     completion:(void (^)(NSError *error, NSString *requestJSON))completion;

/**
 Parse a GET_CRED_DEF response.
 
 @param getCredDefResponse  response json
 @param completion Callback that takes command result as parameter. Cred Def Id and Cred Def json.
 */
+ (void)parseGetCredDefResponse:(NSString *)getCredDefResponse
                     completion:(void (^)(NSError *error, NSString *credDefId, NSString *credDefJson))completion;

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
 Builds a NODE request. Request to add a new node to the pool, or updates existing in the pool.
 
 @param submitterDid DID of the submitter stored in secured Wallet.
 @param targetDid Target Node's DID.  It differs from submitter_did field.
 @param data Data associated with the Node: {
     alias: string - Node's alias
     blskey: string - (Optional) BLS multi-signature key as base58-encoded string.
     client_ip: string - (Optional) Node's client listener IP address.
     client_port: string - (Optional) Node's client listener port.
     node_ip: string - (Optional) The IP address other Nodes use to communicate with this Node.
     node_port: string - (Optional) The port other Nodes use to communicate with this Node.
     services: array<string> - (Optional) The service of the Node. VALIDATOR is the only supported one now.
 }
 @param completion Callback that takes command result as parameter. Return request result as json.
 */
+ (void)buildNodeRequestWithSubmitterDid:(NSString *)submitterDid
                               targetDid:(NSString *)targetDid
                                    data:(NSString *)data
                              completion:(void (^)(NSError *error, NSString *requestJSON))completion;

// MARK: - Txn request

/**
 Builds a GET_TXN request. Request to get any transaction by its seq_no.

 @param submitterDid DID of the request submitter.
 @param data seq_no of transaction in ledger.
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildGetTxnRequestWithSubmitterDid:(NSString *)submitterDid
                                      data:(NSNumber *)data
                                completion:(void (^)(NSError *error, NSString *requestJSON))completion;

// MARK: - Pool config request

/**
 Builds a POOL_CONFIG request. Request to change Pool's configuration.

 @param submitterDid DID of the submitter stored in secured Wallet.
 @param writes Whether any write requests can be processed by the pool
               (if false, then pool goes to read-only state). True by default.
 @param force Whether we should apply transaction (for example, move pool to read-only state)
              without waiting for consensus of this transaction.
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildPoolConfigRequestWithSubmitterDid:(NSString *)submitterDid
                                        writes:(BOOL)writes
                                         force:(BOOL)force
                                    completion:(void (^)(NSError *error, NSString *requestJSON))completion;
// MARK: - Pool upgrade request

/**
 Builds a POOL_UPGRADE request. Request to upgrade the Pool (sent by Trustee).
 It upgrades the specified Nodes (either all nodes in the Pool, or some specific ones).

 @param submitterDid DID of the submitter stored in secured Wallet.
 @param name Human-readable name for the upgrade.
 @param version The version of indy-node package we perform upgrade to.
                Must be greater than existing one (or equal if reinstall flag is True).
 @param action Either start or cancel.
 @param sha256 sha256 hash of the package.
 @param timeout (Optional) Limits upgrade time on each Node.
 @param schedule (Optional) Schedule of when to perform upgrade on each node. Map Node DIDs to upgrade time.
 @param justification (Optional) justification string for this particular Upgrade.
 @param reinstall Whether it's allowed to re-install the same version. False by default.
 @param force Whether we should apply transaction (schedule Upgrade) without waiting for consensus of this transaction.
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
// MARK: - Revocation registry definition request

/**
 Builds a REVOC_REG_DEF request. Request to add the definition of revocation registry
 to an exists credential definition.

 @param submitterDid DID of the submitter stored in secured Wallet.
 @param data Revocation Registry data:
     {
         "id": string - ID of the Revocation Registry,
         "revocDefType": string - Revocation Registry type (only CL_ACCUM is supported for now),
         "tag": string - Unique descriptive ID of the Registry,
         "credDefId": string - ID of the corresponding CredentialDefinition,
         "value": Registry-specific data {
             "issuanceType": string - Type of Issuance(ISSUANCE_BY_DEFAULT or ISSUANCE_ON_DEMAND),
             "maxCredNum": number - Maximum number of credentials the Registry can serve.
             "tailsHash": string - Hash of tails.
             "tailsLocation": string - Location of tails file.
             "publicKeys": <public_keys> - Registry's public key.
         },
         ver: RevRegDef json version
     }
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildRevocRegDefRequestWithSubmitterDid:(NSString *)submitterDid
                                           data:(NSString *)data
                                     completion:(void (^)(NSError *error, NSString *requestJSON))completion;

/**
 Builds a GET_REVOC_REG_DEF request. Request to get a revocation registry definition,
 that Issuer creates for a particular Credential Definition.

 @param submitterDid DID of the submitter stored in secured Wallet.
 @param id ID of the RevocRegDef in ledger.
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildGetRevocRegDefRequestWithSubmitterDid:(NSString *)submitterDid
                                                id:(NSString *)id
                                        completion:(void (^)(NSError *error, NSString *requestJSON))completion;

/**
 Parse a GET_REVOC_REG_DEF response.

 @param getRevocRegDefResponse response json
 @param completion Callback that takes command result as parameter. 
 Returns Revocation Registry Definition Id and Revocation Registry Definition json.
 {
     "id": string - ID of the Revocation Registry,
     "revocDefType": string - Revocation Registry type (only CL_ACCUM is supported for now),
     "tag": string - Unique descriptive ID of the Registry,
     "credDefId": string - ID of the corresponding CredentialDefinition,
     "value": Registry-specific data {
         "issuanceType": string - Type of Issuance(ISSUANCE_BY_DEFAULT or ISSUANCE_ON_DEMAND),
         "maxCredNum": number - Maximum number of credentials the Registry can serve.
         "tailsHash": string - Hash of tails.
         "tailsLocation": string - Location of tails file.
         "publicKeys": <public_keys> - Registry's public key.
     },
     "ver": string
 }
 */
+ (void)parseGetRevocRegDefResponse:(NSString *)getRevocRegDefResponse
                         completion:(void (^)(NSError *error, NSString *revocRegDefId, NSString *revocRegDefJson))completion;


// MARK: - Revocation registry entry request

/**
 Builds a REVOC_REG_ENTRY request.  Request to add the RevocReg entry containing
 the new accumulator value and issued/revoked indices.
 This is just a delta of indices, not the whole list.
 So, it can be sent each time a new claim is issued/revoked.

 @param submitterDid DID of the submitter stored in secured Wallet.
 @param type Revocation Registry type (only CL_ACCUM is supported for now).
 @param revocRegDefId ID of the corresponding RevocRegDef.
 @param value Registry-specific data: {
     value: {
         prevAccum: string - previous accumulator value.
         accum: string - current accumulator value.
         issued: array<number> - an array of issued indices.
         revoked: array<number> an array of revoked indices.
     },
     ver: RevocRegEntry json version
     
 }
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildRevocRegEntryRequestWithSubmitterDid:(NSString *)submitterDid
                                             type:(NSString *)type
                                    revocRegDefId:(NSString *)revocRegDefId
                                            value:(NSString *)value
                                       completion:(void (^)(NSError *error, NSString *requestJSON))completion;

/**
 Builds a GET_REVOC_REG request. Request to get the accumulated state of the Revocation Registry
 by ID. The state is defined by the given timestamp.

 @param submitterDid DID of the submitter stored in secured Wallet.
 @param revocRegDefId ID of the corresponding RevocRegDef in ledger.
 @param timestamp Requested time represented as a total number of seconds from Unix Epoch
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildGetRevocRegRequestWithSubmitterDid:(NSString *)submitterDid
                                  revocRegDefId:(NSString *)revocRegDefId
                                      timestamp:(NSNumber *)timestamp
                                     completion:(void (^)(NSError *error, NSString *requestJSON))completion;


/**
 Parse a GET_REVOC_REG response.

 @param getRevocRegResponse response json
 @param completion Callback that takes command result as parameter. 
 Returns Revocation Registry Definition Id, Revocation Registry json and Timestamp.
 {
     "value": Registry-specific data {
         "accum": string - Type of Issuance(ISSUANCE_BY_DEFAULT or ISSUANCE_ON_DEMAND),
     },
     "ver": string
 }
 */
+ (void)parseGetRevocRegResponse:(NSString *)getRevocRegResponse
                      completion:(void (^)(NSError *error, NSString *revocRegDefId, NSString *revocRegJson, NSNumber *timestamp))completion;

/**
 Builds a GET_REVOC_REG_DELTA request. Request to get the delta of the accumulated state of the Revocation Registry.
 The Delta is defined by from and to timestamp fields.
 If from is not specified, then the whole state till to will be returned.

 @param submitterDid DID of the submitter stored in secured Wallet.
 @param revocRegDefId ID of the corresponding RevocRegDef in ledger.
 @param from Requested time represented as a total number of seconds from Unix Epoch
 @param to Requested time represented as a total number of seconds from Unix Epoch
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildGetRevocRegDeltaRequestWithSubmitterDid:(NSString *)submitterDid
                                       revocRegDefId:(NSString *)revocRegDefId
                                                from:(NSNumber *)from
                                                  to:(NSNumber *)to
                                          completion:(void (^)(NSError *error, NSString *requestJSON))completion;


/**
 Parse a GET_REVOC_REG_DELTA response.

 @param getRevocRegDeltaResponse response json
 @param completion Callback that takes command result as parameter. 
 Returns evocation Registry Definition Id, Revocation Registry Delta json and Timestamp.
 {
     "value": Registry-specific data {
         prevAccum: string - previous accumulator value.
         accum: string - current accumulator value.
         issued: array<number> - an array of issued indices.
         revoked: array<number> an array of revoked indices.
     },
     "ver": string
 }
 */
+ (void)parseGetRevocRegDeltaResponse:(NSString *)getRevocRegDeltaResponse
                           completion:(void (^)(NSError *error, NSString *revocRegDefId, NSString *revocRegDeltaJson, NSNumber *timestamp))completion;

@end
