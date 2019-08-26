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
 Multi signs request message.

 Adds submitter information to passed request json, signs it with submitter
 sign key.

 @param requestJson Request data json.
 @param submitterDid Id of Identity stored in secured Wallet.
 @param walletHandle Wallet handle (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter. Returns signed request json.
 */
+ (void)multiSignRequest:(NSString *)requestJson
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

/**
 Send action to particular nodes of validator pool.

 The list of requests can be send:
    POOL_RESTART
    GET_VALIDATOR_INFO

 The request is sent to the nodes as is. It's assumed that it's already prepared.

 @param requestJson Request data json.
 @param nodes (Optional) List of node names to send the request.
           ["Node1", "Node2",...."NodeN"]
 @param timeout (Optional) Time to wait respond from nodes (override the default timeout) (in sec).
 @param poolHandle Pool handle (created by IndyPool::openPoolLedgerWithName).
 @param completion Callback that takes command result as parameter. Returns signed request json.
 */
+ (void)submitAction:(NSString *)requestJson
               nodes:(NSString *)nodes
             timeout:(NSNumber *)timeout
          poolHandle:(IndyHandle)poolHandle
          completion:(void (^)(NSError *error, NSString *requestResultJSON))completion;

// MARK: - Nym request

/**
 Builds a NYM request. Request to create a new NYM record for a specific user.

 @param submitterDid Identifier (DID) of the transaction author as base58-encoded string.
                     Actual request sender may differ if Endorser is used (look at `appendEndorserToRequest`)
 @param targetDid Target DID as base58-encoded string for 16 or 32 bit DID value.
 @param verkey Target identity verification key as base58-encoded string.
 @param alias NYM's alias.
 @param role Role of a user NYM record:
                                 null (common USER)
                                 TRUSTEE
                                 STEWARD
                                 TRUST_ANCHOR
                                 ENDORSER - equal to TRUST_ANCHOR that will be removed soon
                                 NETWORK_MONITOR
                                 empty string to reset role
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
 
 @param submitterDid (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
 @param targetDid Target DID as base58-encoded string for 16 or 32 bit DID value.
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildGetNymRequestWithSubmitterDid:(NSString *)submitterDid
                                 targetDID:(NSString *)targetDid
                                completion:(void (^)(NSError *error, NSString *requestJSON))completion;

// MARK: - Attribute request

/**
 Builds an ATTRIB request. Request to add attribute to a NYM record.

 @param submitterDid Identifier (DID) of the transaction author as base58-encoded string.
                     Actual request sender may differ if Endorser is used (look at `appendEndorserToRequest`)
 @param targetDid Target DID as base58-encoded string for 16 or 32 bit DID value.
 @param raw (Optional) Json, where key is attribute name and value is attribute value.
 @param hash (Optional) Hash of attribute data.
 @param enc (Optional) (Optional) Encrypted value attribute data.
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
 
 @param submitterDid (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
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

// MARK: - Get Validator Info request

/**
 Builds GET_VALIDATOR_INFO request.
 
 @param submitterDid Id of Identity stored in secured Wallet.
 @param completion Callback that takes command result as parameter. Request result as json.
 */
+ (void)buildGetValidatorInfo:(NSString *)submitterDid
                   completion:(void (^)(NSError *error, NSString *requestJSON))completion;

// MARK: - Schema request

/**
 Builds a SCHEMA request. Request to add Credential's schema.
 
 @param submitterDid Identifier (DID) of the transaction author as base58-encoded string.
                     Actual request sender may differ if Endorser is used (look at `appendEndorserToRequest`)
 @param data  Credential schema.
              {
                  id: identifier of schema
                  attrNames: array of attribute name strings (the number of attributes should be less or equal than 125)
                  name: Schema's name string
                  version: Schema's version string,
                  ver: Version of the Schema json
              }
 @param completion Callback that takes command result as parameter. Request result as json.
 */
+ (void)buildSchemaRequestWithSubmitterDid:(NSString *)submitterDid
                                      data:(NSString *)data
                                completion:(void (^)(NSError *error, NSString *requestJSON))completion;

/**
 Builds a GET_SCHEMA request. Request to get Credential's Schema.
 
 @param submitterDid (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
 @param id Schema ID in ledger
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildGetSchemaRequestWithSubmitterDid:(NSString *)submitterDid
                                           id:(NSString *)id
                                   completion:(void (^)(NSError *error, NSString *requestJSON))completion;

/**
 Parse a GET_SCHEMA response to get Schema in the format compatible with Anoncreds API
 
 @param getSchemaResponse  response of GET_SCHEMA request.
 @param completion Callback that takes command result as parameter.
 Returns Schema Id and Schema json.
     {
         id: identifier of schema
         attrNames: array of attribute name strings
         name: Schema's name string
         version: Schema's version string
         ver: Version of the Schema json
     }
 */
+ (void)parseGetSchemaResponse:(NSString *)getSchemaResponse
                    completion:(void (^)(NSError *error, NSString *schemaId, NSString *schemaJson))completion;

// MARK: - CredDefRequest request

/**
 Builds an CRED_DEF request. Request to add a Credential Definition (in particular, public key),
 that Issuer creates for a particular Credential Schema.
 
 @param submitterDid Identifier (DID) of the transaction author as base58-encoded string.
                     Actual request sender may differ if Endorser is used (look at `appendEndorserToRequest`)
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
 
 @param submitterDid (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
 @param id Credential Definition ID in ledger
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildGetCredDefRequestWithSubmitterDid:(NSString *)submitterDid
                                            id:(NSString *)id
                                    completion:(void (^)(NSError *error, NSString *requestJSON))completion;

/**
 Parse a GET_CRED_DEF response to get Credential Definition in the format compatible with Anoncreds API.
 
 @param getCredDefResponse  response of GET_CRED_DEF request.
 @param completion Callback that takes command result as parameter. 
 Returns Credential Definition Id and Credential Definition json.
  {
      id: string - identifier of credential definition
      schemaId: string - identifier of stored in ledger schema
      type: string - type of the credential definition. CL is the only supported type now.
      tag: string - allows to distinct between credential definitions for the same issuer and schema
      value: Dictionary with Credential Definition's data: {
          primary: primary credential public key,
          Optional<revocation>: revocation credential public key
      },
      ver: Version of the Credential Definition json
  }
 */
+ (void)parseGetCredDefResponse:(NSString *)getCredDefResponse
                     completion:(void (^)(NSError *error, NSString *credDefId, NSString *credDefJson))completion;

// MARK: - Ddo request

/**
 Builds a request to get a DDO.

 
 @param submitterDid (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
 @param targetDid Id of Identity stored in secured Wallet.
 @param completion Callback that takes command result as parameter. Returns result as json.
 */
+ (void)buildGetDdoRequestWithSubmitterDid:(NSString *)submitterDid
                                 targetDID:(NSString *)targetDid
                                completion:(void (^)(NSError *error, NSString *requestResultJSON))completion;

// MARK: - Node request

/**
 Builds a NODE request. Request to add a new node to the pool, or updates existing in the pool.
 
 @param submitterDid Identifier (DID) of the transaction author as base58-encoded string.
                     Actual request sender may differ if Endorser is used (look at `appendEndorserToRequest`)
 @param targetDid Target Node's DID.  It differs from submitter_did field.
 @param data Data associated with the Node: Data associated with the Node:
 {
     alias: string - Node's alias
     blskey: string - (Optional) BLS multi-signature key as base58-encoded string.
     blskey_pop: string - (Optional) BLS key proof of possession as base58-encoded string.
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

 @param submitterDid (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
 @param ledgerType (Optional) type of the ledger the requested transaction belongs to:
          DOMAIN - used default,
          POOL,
          CONFIG
          any number
 @param data seq_no of transaction in ledger.
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildGetTxnRequestWithSubmitterDid:(NSString *)submitterDid
                                ledgerType:(NSString *)ledgerType
                                      data:(NSNumber *)data
                                completion:(void (^)(NSError *error, NSString *requestJSON))completion;

// MARK: - Pool config request

/**
 Builds a POOL_CONFIG request. Request to change Pool's configuration.

 @param submitterDid Identifier (DID) of the transaction author as base58-encoded string.
                     Actual request sender may differ if Endorser is used (look at `appendEndorserToRequest`)
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

// MARK: - Pool restart request

/**
 Builds a POOL_RESTART request.

 @param submitterDid Identifier (DID) of the transaction author as base58-encoded string.
                     Actual request sender may differ if Endorser is used (look at `appendEndorserToRequest`)
 @param action  - Action that pool has to do after received transaction.
        Can be "start" or "cancel".
 @param datetime - Time when pool must be restarted
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildPoolRestartRequestWithSubmitterDid:(NSString *)submitterDid
                                         action:(NSString *)action
                                       datetime:(NSString *)datetime
                                     completion:(void (^)(NSError *error, NSString *requestJSON))completion;

// MARK: - Pool upgrade request

/**
 Builds a POOL_UPGRADE request. Request to upgrade the Pool (sent by Trustee).
 It upgrades the specified Nodes (either all nodes in the Pool, or some specific ones).

 @param submitterDid Identifier (DID) of the transaction author as base58-encoded string.
                     Actual request sender may differ if Endorser is used (look at `appendEndorserToRequest`)
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
 @param package_ (Optional) Package to be upgraded.
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
                                       package_:(NSString *)package_
                                     completion:(void (^)(NSError *error, NSString *requestJSON))completion;
// MARK: - Revocation registry definition request

/**
 Builds a REVOC_REG_DEF request. Request to add the definition of revocation registry
 to an exists credential definition.

 @param submitterDid Identifier (DID) of the transaction author as base58-encoded string.
                     Actual request sender may differ if Endorser is used (look at `appendEndorserToRequest`)
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
      "ver": string - version of revocation registry definition json.
  }
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildRevocRegDefRequestWithSubmitterDid:(NSString *)submitterDid
                                           data:(NSString *)data
                                     completion:(void (^)(NSError *error, NSString *requestJSON))completion;

/**
 Builds a GET_REVOC_REG_DEF request. Request to get a revocation registry definition,
 that Issuer creates for a particular Credential Definition.

 @param submitterDid (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
 @param id ID of Revocation Registry Definition in ledger.
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildGetRevocRegDefRequestWithSubmitterDid:(NSString *)submitterDid
                                                id:(NSString *)id
                                        completion:(void (^)(NSError *error, NSString *requestJSON))completion;

/**
 Parse a GET_REVOC_REG_DEF response to get Revocation Registry Definition in the format compatible with Anoncreds API.

 @param getRevocRegDefResponse response of GET_REVOC_REG_DEF request.
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
      "ver": string - version of revocation registry definition json.
  }
 */
+ (void)parseGetRevocRegDefResponse:(NSString *)getRevocRegDefResponse
                         completion:(void (^)(NSError *error, NSString *revocRegDefId, NSString *revocRegDefJson))completion;


// MARK: - Revocation registry entry request

/**
 Builds a REVOC_REG_ENTRY request.  Request to add the RevocReg entry containing
 the new accumulator value and issued/revoked indices.
 This is just a delta of indices, not the whole list.
 So, it can be sent each time a new credential is issued/revoked.

 @param submitterDid Identifier (DID) of the transaction author as base58-encoded string.
                     Actual request sender may differ if Endorser is used (look at `appendEndorserToRequest`)
 @param type Revocation Registry type (only CL_ACCUM is supported for now).
 @param revocRegDefId ID of the corresponding RevocRegDef.
 @param value Registry-specific data: 
   {
       value: {
           prevAccum: string - previous accumulator value.
           accum: string - current accumulator value.
           issued: array<number> - an array of issued indices.
           revoked: array<number> an array of revoked indices.
       },
       ver: string - version revocation registry entry json
  
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

 @param submitterDid (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
 @param revocRegDefId ID of the corresponding Revocation Registry Definition in ledger.
 @param timestamp Requested time represented as a total number of seconds from Unix Epoch
 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildGetRevocRegRequestWithSubmitterDid:(NSString *)submitterDid
                                  revocRegDefId:(NSString *)revocRegDefId
                                      timestamp:(NSNumber *)timestamp
                                     completion:(void (^)(NSError *error, NSString *requestJSON))completion;


/**
 Parse a GET_REVOC_REG response to get Revocation Registry in the format compatible with Anoncreds API.

 @param getRevocRegResponse response of GET_REVOC_REG request.
 @param completion Callback that takes command result as parameter. 
 Returns Revocation Registry Definition Id, Revocation Registry json and Timestamp.
  {
      "value": Registry-specific data {
          "accum": string - current accumulator value.
      },
      "ver": string - version revocation registry json
  }
 */
+ (void)parseGetRevocRegResponse:(NSString *)getRevocRegResponse
                      completion:(void (^)(NSError *error, NSString *revocRegDefId, NSString *revocRegJson, NSNumber *timestamp))completion;

/**
 Builds a GET_REVOC_REG_DELTA request. Request to get the delta of the accumulated state of the Revocation Registry.
 The Delta is defined by from and to timestamp fields.
 If from is not specified, then the whole state till to will be returned.

 @param submitterDid (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
 @param revocRegDefId ID of the corresponding Revocation Registry Definition in ledger.
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
 Parse a GET_REVOC_REG_DELTA response to get Revocation Registry Delta in the format compatible with Anoncreds API.

 @param getRevocRegDeltaResponse response of GET_REVOC_REG_DELTA request.
 @param completion Callback that takes command result as parameter. 
 Returns evocation Revocation Registry Definition Id, Revocation Registry Delta json and Timestamp.
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

// MARK: - Auth Rule request

/**
 Builds a AUTH_RULE request. Request to change authentication rules for a ledger transaction.

 @param submitterDid Identifier (DID) of the transaction author as base58-encoded string.
                     Actual request sender may differ if Endorser is used (look at `appendEndorserToRequest`)
 @param txnType - ledger transaction alias or associated value.
 @param action - type of an action.
          Can be either "ADD" (to add a new rule) or "EDIT" (to edit an existing one).
 @param field - transaction field.
 @param oldValue - (Optional) old value of a field, which can be changed to a new_value (mandatory for EDIT action).
 @param newValue - (Optional) new value that can be used to fill the field.
 @param constraint - set of constraints required for execution of an action in the following format:
        {
            constraint_id - <string> type of a constraint.
                Can be either "ROLE" to specify final constraint or  "AND"/"OR" to combine constraints.
            role - <string> (optional) role of a user which satisfy to constrain.
            sig_count - <u32> the number of signatures required to execution action.
            need_to_be_owner - <bool> if user must be an owner of transaction (false by default).
            off_ledger_signature - <bool> (optional) allow signature of unknow for ledger did (false by default).
            metadata - <object> (optional) additional parameters of the constraint.
        }
      can be combined by
        {
            'constraint_id': <"AND" or "OR">
            'auth_constraints': [<constraint_1>, <constraint_2>]
        }

 Default ledger auth rules: https://github.com/hyperledger/indy-node/blob/master/docs/source/auth_rules.md

 More about AUTH_RULE request: https://github.com/hyperledger/indy-node/blob/master/docs/source/requests.md#auth_rule

 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildAuthRuleRequestWithSubmitterDid:(NSString *)submitterDid
                                     txnType:(NSString *)txnType
                                      action:(NSString *)action
                                       field:(NSString *)field
                                    oldValue:(NSString *)oldValue
                                    newValue:(NSString *)newValue
                                  constraint:(NSString *)constraint
                                  completion:(void (^)(NSError *error, NSString *requestJSON))completion;


/**
 Builds a AUTH_RULES request. Request to change multiple authentication rules for a ledger transaction.

 @param submitterDid Identifier (DID) of the transaction author as base58-encoded string.
                     Actual request sender may differ if Endorser is used (look at `appendEndorserToRequest`)
 @param data - a list of auth rules: [
        {
            "auth_type": ledger transaction alias or associated value,
            "auth_action": type of an action,
            "field": transaction field,
            "old_value": (Optional) old value of a field, which can be changed to a new_value (mandatory for EDIT action),
            "new_value": (Optional) new value that can be used to fill the field,
            "constraint": set of constraints required for execution of an action in the format described above for `indy_build_auth_rule_request` function.
        },
        ...
    ]

 Default ledger auth rules: https://github.com/hyperledger/indy-node/blob/master/docs/source/auth_rules.md

 More about AUTH_RULE request: https://github.com/hyperledger/indy-node/blob/master/docs/source/requests.md#auth_rules

 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildAuthRulesRequestWithSubmitterDid:(NSString *)submitterDid
                                         data:(NSString *)data
                                   completion:(void (^)(NSError *error, NSString *requestJSON))completion;

/**
 Builds a GET_AUTH_RULE request. Request to get authentication rules for ledger transactions.

 NOTE: Either none or all transaction related parameters must be specified (`old_value` can be skipped for `ADD` action).
     * none - to get all authentication rules for all ledger transactions
     * all - to get authentication rules for specific action (`old_value` can be skipped for `ADD` action)

 @param submitterDid (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
 @param txnType - (Optional) target ledger transaction alias or associated value.
 @param action - (Optional) target action type. Can be either "ADD" or "EDIT".
 @param field - (Optional) target transaction field.
 @param oldValue - (Optional) old value of field, which can be changed to a new_value (mandatory for EDIT action).
 @param newValue - (Optional) new value that can be used to fill the field.

 @param completion Callback that takes command result as parameter. Returns request result as json.
 */
+ (void)buildGetAuthRuleRequestWithSubmitterDid:(NSString *)submitterDid
                                        txnType:(NSString *)txnType
                                         action:(NSString *)action
                                          field:(NSString *)field
                                       oldValue:(NSString *)oldValue
                                       newValue:(NSString *)newValue
                                     completion:(void (^)(NSError *error, NSString *requestJSON))completion;

/**
 Parse transaction response to fetch metadata.
 The important use case for this method is validation of Node's response freshens.

 Distributed Ledgers can reply with outdated information for consequence read request after write.
 To reduce pool load libindy sends read requests to one random node in the pool.
 Consensus validation is performed based on validation of nodes multi signature for current ledger Merkle Trie root.
 This multi signature contains information about the latest ldeger's transaction ordering time and sequence number that this method returns.

 If node that returned response for some reason is out of consensus and has outdated ledger
 it can be caught by analysis of the returned latest ledger's transaction ordering time and sequence number.

 There are two ways to filter outdated responses:
     1) based on "seqNo" - sender knows the sequence number of transaction that he consider as a fresh enough.
     2) based on "txnTime" - sender knows the timestamp that he consider as a fresh enough.

 Note: response of GET_VALIDATOR_INFO request isn't supported

 @param response esponse of write or get request.
 @param completion Callback that takes command result as parameter.
 Returns Response Metadata
     {
         "seqNo": Option<u64> - transaction sequence number,
         "txnTime": Option<u64> - transaction ordering time,
         "lastSeqNo": Option<u64> - the latest transaction seqNo for particular Node,
         "lastTxnTime": Option<u64> - the latest transaction ordering time for particular Node
     }
 */
+ (void)getResponseMetadata:(NSString *)response
                 completion:(void (^)(NSError *error, NSString *responseMetadata))completion;

/**
 Builds a TXN_AUTHR_AGRMT request. Request to add a new version of Transaction Author Agreement to the ledger.

 EXPERIMENTAL

 @param submitterDid Identifier (DID) of the transaction author as base58-encoded string.
                     Actual request sender may differ if Endorser is used (look at `appendEndorserToRequest`)
 @param text a content of the TTA.
 @param version a version of the TTA (unique UTF-8 string).
 
 Returns Request result as json.
 */
+ (void)buildTxnAuthorAgreementRequestWithSubmitterDid:(NSString *)submitterDid
                                                  text:(NSString *)text
                                               version:(NSString *)version
                                            completion:(void (^)(NSError *error, NSString *responseMetadata))completion;

/**
 Builds a GET_TXN_AUTHR_AGRMT request. Request to get a specific Transaction Author Agreement from the ledger.

 EXPERIMENTAL

 @param submitterDid (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
 @param data (Optional) specifies a condition for getting specific TAA.
 Contains 3 mutually exclusive optional fields:
 {
     hash: Optional<str> - hash of requested TAA,
     version: Optional<str> - version of requested TAA.
     timestamp: Optional<u64> - ledger will return TAA valid at requested timestamp.
 }
 Null data or empty JSON are acceptable here. In this case, ledger will return the latest version of TAA.
 
 Returns Request result as json.
 */
+ (void)buildGetTxnAuthorAgreementRequestWithSubmitterDid:(NSString *)submitterDid
                                                     data:(NSString *)data
                                               completion:(void (^)(NSError *error, NSString *responseMetadata))completion;

/**
 Builds a SET_TXN_AUTHR_AGRMT_AML request. Request to add a new list of acceptance mechanisms for transaction author agreement.
 Acceptance Mechanism is a description of the ways how the user may accept a transaction author agreement.
 
 EXPERIMENTAL

 @param submitterDid Identifier (DID) of the transaction author as base58-encoded string.
                     Actual request sender may differ if Endorser is used (look at `appendEndorserToRequest`)
 @param aml a set of new acceptance mechanisms:
 {
     “<acceptance mechanism label 1>”: { acceptance mechanism description 1},
     “<acceptance mechanism label 2>”: { acceptance mechanism description 2},
     ...
 }
 @param version a version of new acceptance mechanisms. (Note: unique on the Ledger).
 @param amlContext (Optional) common context information about acceptance mechanisms (may be a URL to external resource).

 Returns Request result as json.
 */
+ (void)buildAcceptanceMechanismsRequestWithSubmitterDid:(NSString *)submitterDid
                                                     aml:(NSString *)aml
                                                 version:(NSString *)version
                                              amlContext:(NSString *)amlContext
                                              completion:(void (^)(NSError *error, NSString *responseMetadata))completion;

/**
 Builds a GET_TXN_AUTHR_AGRMT_AML request. Request to get a list of  acceptance mechanisms from the ledger
 valid for specified time or the latest one.
 
 EXPERIMENTAL

 @param submitterDid (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
 @param timestamp time to get an active acceptance mechanisms. The latest one will be returned for nil.
 @param version (Optional) version of acceptance mechanisms.

 NOTE: timestamp and version cannot be specified together.

 Returns Request result as json.
 */
+ (void)buildGetAcceptanceMechanismsRequestWithSubmitterDid:(NSString *)submitterDid
                                                  timestamp:(NSNumber *)timestamp
                                                    version:(NSString *)version
                                                 completion:(void (^)(NSError *error, NSString *responseMetadata))completion;

/**
 Append transaction author agreement acceptance data to a request.
 This function should be called before signing and sending a request
 if there is any transaction author agreement set on the Ledger.

 EXPERIMENTAL

 This function may calculate hash by itself or consume it as a parameter.
 If all text, version and taaDigest parameters are specified, a check integrity of them will be done.

 @param requestJson original request data json.
 @param text (Optional) raw data about TAA from ledger.
 @param version (Optional) version of TAA from ledger.
     text and version should be passed together.
     text and version are required if taaDigest parameter is omitted.
 @param taaDigest (Optional) hash on text and version. This parameter is required if text and version parameters are omitted.
 @param accMechType mechanism how user has accepted the TAA
 @param timeOfAcceptance UTC timestamp when user has accepted the TAA. Note that the time portion will be discarded to avoid a privacy risk.

 Returns Updated request result as json.
 */
+ (void)appendTxnAuthorAgreementAcceptanceToRequest:(NSString *)requestJson
                                               text:(NSString *)text
                                            version:(NSString *)version
                                          taaDigest:(NSString *)taaDigest
                                        accMechType:(NSString *)accMechType
                                   timeOfAcceptance:(NSNumber *)timeOfAcceptance
                                         completion:(void (^)(NSError *error, NSString *responseMetadata))completion;

/**
 Append Endorser to an existing request.

 An author of request still is a `DID` used as a `submitter_did` parameter for the building of the request.
 But it is expecting that the transaction will be sent by the specified Endorser.

 Note: Both Transaction Author and Endorser must sign output request after that.

 More about Transaction Endorser: https://github.com/hyperledger/indy-node/blob/master/design/transaction_endorser.md
                                  https://github.com/hyperledger/indy-sdk/blob/master/docs/configuration.md

 @param requestJson original request data json.
 @param endorserDid DID of the Endorser that will submit the transaction.
                    The Endorser's DID must be present on the ledger.

 Returns Updated request result as json.
 */
+ (void)appendEndorserToRequest:(NSString *)requestJson
                    endorserDid:(NSString *)endorserDid
                     completion:(void (^)(NSError *error, NSString *outRequestJson))completion;

@end
