//
//  IndyAnoncreds.h
//  libindy
//


#import <Foundation/Foundation.h>
#import "IndyTypes.h"

@interface IndyAnoncreds : NSObject

/**
 Create credential schema entity that describes credential attributes list and allows credentials
 interoperability.

 Schema is public and intended to be shared with all anoncreds workflow actors usually by publishing SCHEMA transaction
 to Indy distributed ledger.
 
 It is IMPORTANT for current version POST Schema in Ledger and after that GET it from Ledger
 with correct seq_no to save compatibility with Ledger.
 After that can call indy_issuer_create_and_store_credential_def to build corresponding Credential Definition.
 
 @param issuerDID DID of schema issuer
 @param name a name the schema
 @param version a version of the schema
 @param attrs a list of schema attributes descriptions
 @param completion Callback that takes command result as parameter. Returns schemaId and schemaJson.
*/
+ (void)issuerCreateSchemaWithName:(NSString *)name
                           version:(NSString *)version
                             attrs:(NSString *)attrs
                         issuerDID:(NSString *)issuerDID
                        completion:(void (^)(NSError *error, NSString *schemaId, NSString *schemaJSON))completion;

/**
 Create credential definition entity that encapsulates credentials issuer DID, credential schema, secrets used for signing credentials
 and secrets used for credentials revocation.

 Credential definition entity contains private and public parts. Private part will be stored in the wallet. Public part
 will be returned as json intended to be shared with all anoncreds workflow actors usually by publishing CRED_DEF transaction
 to Indy distributed ledger.
 
 It is IMPORTANT for current version GET Schema from Ledger with correct seq_no to save compatibility with Ledger.
 
 @param issuerDID DID of the issuer signing credential_def transaction to the Ledger
 @param schemaJSON Schema as a json
 @param tag: allows to distinct between credential definitions for the same issuer and schema
 @param type: type_: credential definition type (optional, 'CL' by default) that defines credentials signature and revocation math.
 Supported types are:
    - 'CL': Camenisch-Lysyanskaya credential signature type
 @param configJSON: type-specific configuration of credential definition as json:
 - 'CL':
   - revocationSupport: whether to request non-revocation credential (optional, default false)
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter. 
 Returns:
    credDefId: identifier of created credential definition.
    credDefJson: public part of created credential definition
*/
+ (void)issuerCreateAndStoreCredentialDefForSchema:(NSString *)schemaJSON
                                         issuerDID:(NSString *)issuerDID
                                               tag:(NSString *)tag
                                              type:(NSString *)type
                                        configJSON:(NSString *)configJSON
                                      walletHandle:(IndyHandle)walletHandle
                                        completion:(void (^)(NSError *error, NSString *credDefId, NSString *credDefJSON))completion;

/**
 Creates a new revocation registry for the given credential definition.
 Stores it in a secure wallet identifying by the returned key.
 
 @param walletHandle: wallet handler (created by open_wallet).
 @param issuerDID: a DID of the issuer signing transaction to the Ledger
 @param type: (optional) registry type. Currently only 'CL_ACCUM' is supported.
 @param tag:
 @param credDefID: id of stored in ledger credential definition
 @param configJSON: {
     "issuance_type": (optional) type of issuance. Currently supported:
         1) ISSUANCE_BY_DEFAULT: all indices are assumed to be issued and initial accumulator is calculated over all indices;
                                 Revocation Registry is updated only during revocation.
         2) ISSUANCE_ON_DEMAND: nothing is issued initially accumulator is 1 (used by default);
     "max_cred_num": maximum number of credentials the new registry can process.
 }
 @param tailsWriterHandle:
 @param tailsWriterConfig:
 @param completion Callback that takes command result as parameter.
 Returns revocation registry definition json and revocation registry entry json.
 */
+ (void)issuerCreateAndStoreRevocRegForCredentialDefId:(NSString *)credDefID
                                             issuerDID:(NSString *)issuerDID
                                                  type:(NSString *)type
                                                   tag:(NSString *)tag
                                            configJSON:(NSString *)configJSON
                                     tailsWriterHandle:(IndyHandle)tailsWriterHandle
                                          walletHandle:(IndyHandle)walletHandle
                                            completion:(void (^)(NSError *error, NSString *revocRegID, NSString *revocRegDefJSON, NSString *revocRegEntryJSON))completion;

/**
 Create credential offer and store it in wallet.

 @param cred_def_id: id of stored in ledger credential definition
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter.
 Returns credential offer json:
  {
      "schema_id": string,
      "cred_def_id": string,
      // Fields below can depend on Cred Def type
      "nonce": string,
      "key_correctness_proof" : <key_correctness_proof>
  }
*/
+ (void)issuerCreateCredentialOfferForCredDefId:(NSString *)credDefID
                                   walletHandle:(IndyHandle)walletHandle
                                     completion:(void (^)(NSError *error, NSString *credentialOfferJSON))completion;

/**
 Check Cred Request for the given Cred Offer and issue Credential for the given Cred Request.

 Cred Request must match Cred Offer. The credential definition and revocation registry definition
 referenced in Cred Offer and Cred Request must be already created and stored into the wallet.

 Information for this credential revocation will be store in the wallet as part of revocation registry under
 generated cred_revoc_id local for this wallet.

 This call returns revoc registry delta as json file intended to be shared as REVOC_REG_ENTRY transaction.
 Note that it is possible to accumulate deltas to reduce ledger load.

 @param  credOfferJSON: a cred offer created by issuerCreateCredentialOfferForCredDefId
 @param  credRequestJSON: a credential request created by proverCreateCredentialReqForCredentialOffer
 @param  credValuesJSON: a credential containing attribute values for each of requested attribute names.
     Example:
     {
      "attr1" : {"raw": "value1", "encoded": "value1_as_int" },
      "attr2" : {"raw": "value1", "encoded": "value1_as_int" }
     }
 @param revRegId: (Optional) id of stored in ledger revocation registry definition
 @param blobStorageReaderHandle: (Optional) Pre-configured blob storage reader instance handle that will allow to read revocation tails
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter. 
 Return:
     credJSON: Credential json containing signed credential values
        {
            "schema_id": string,
            "cred_def_id": string,
            "rev_reg_def_id", Optional<string>,
            "values": <see cred_values_json above>,
            // Fields below can depend on Cred Def type
            "signature": <signature>,
            "signature_correctness_proof": <signature_correctness_proof>
        }
     credRevocID: local id for revocation info (Can be used for revocation of this cred)
     revocRegDeltaJSON: Revocation registry delta json with a newly issued credential
 */
+ (void)issuerCreateCredentialForCredentialRequest:(NSString *)credReqJSON
                                     credOfferJSON:(NSString *)credOfferJSON
                                    credValuesJSON:(NSString *)credValuesJSON
                                          revRegId:(NSString *)revRegId
                           blobStorageReaderHandle:(NSNumber *)blobStorageReaderHandle
                                      walletHandle:(IndyHandle)walletHandle
                                        completion:(void (^)(NSError *error, NSString *credJSON, NSString *credRevocID, NSString *revocRegDeltaJSON))completion;

/**
 Revoke a credential identified by a cred_revoc_id (returned by issuerCreateCredentialForCredentialRequest).

 The corresponding credential definition and revocation registry must be already
 created an stored into the wallet.

 This call returns revoc registry delta as json file intended to be shared as REVOC_REG_ENTRY transaction.
 Note that it is possible to accumulate deltas to reduce ledger load.

 @param revRegId: id of revocation registry stored in wallet
 @param blobStorageReaderHandle: pre-configured blob storage reader instance handle that will allow to read revocation tails
 @param credRevocId: local id for revocation info
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter. 
 Returns Revocation registry delta json with a revoked credential
 */
+ (void)issuerRevokeCredentialByCredRevocId:(NSString *)credRevocId
                                   revRegId:(NSString *)revRegId
                    blobStorageReaderHandle:(NSNumber *)blobStorageReaderHandle
                               walletHandle:(IndyHandle)walletHandle
                                 completion:(void (^)(NSError *error, NSString *revocRegDeltaJSON))completion;

/*+ (void)issuerRecoverCredentialByCredRevocId:(NSString *)credRevocId
                                    revRegId:(NSString *)revRegId
                     blobStorageReaderHandle:(NSNumber *)blobStorageReaderHandle
                                walletHandle:(IndyHandle)walletHandle
                                  completion:(void (^)(NSError *error, NSString *revocRegDeltaJSON))completion;*/

/**
 Merge two revocation registry deltas (returned by issuerCreateCredentialForCredentialRequest or issuerRevokeCredentialByCredRevocId) to accumulate common delta.
 Send common delta to ledger to reduce the load.

 @param revRegDelta: revocation registry delta.
 @param otherRevRegDelta: revocation registry delta for which PrevAccum value  is equal to current accum value of revRegDelta.
 @param completion Callback that takes command result as parameter. 
 Returns merged revocation registry delta
 */
+ (void)issuerMergerRevocationRegistryDelta:(NSString *)revRegDelta
                                  withDelta:(NSString *)otherRevRegDelta
                                 completion:(void (^)(NSError *error, NSString *credOfferJSON))completion;

/**
 Creates a master secret with a given name and stores it in the wallet.
 The name must be unique.
 
 @param masterSecretID (optional, if not present random one will be generated) new master id
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Returns error code and id of generated master secret.
 
 */
+ (void)proverCreateMasterSecret:(NSString *)masterSecretID
                    walletHandle:(IndyHandle)walletHandle
                      completion:(void (^)(NSError *error, NSString *outMasterSecretId))completion;

/**
 
 Creates a clam request for the given credential offer.

 The method creates a blinded master secret for a master secret identified by a provided name.
 The master secret identified by the name must be already stored in the secure wallet (see prover_create_master_secret)
 The blinded master secret is a part of the credential request.

 @param proverDID: a DID of the prover
 @param credOfferJSON: credential offer as a json containing information about the issuer and a credential
 @param credentialDefJSON: credential definition json
 @param masterSecretID: the id of the master secret stored in the wallet
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter. 
 Returns 
     credReqJSON: Credential request json for creation of credential by Issuer
     {
      "prover_did" : string,
      "cred_def_id" : string,
         // Fields below can depend on Cred Def type
      "blinded_ms" : <blinded_master_secret>,
      "blinded_ms_correctness_proof" : <blinded_ms_correctness_proof>,
      "nonce": string
    }
    credReqMetadataJSON: Credential request metadata json for processing of received form Issuer credential.
 */
+ (void)proverCreateCredentialReqForCredentialOffer:(NSString *)credOfferJSON
                                  credentialDefJSON:(NSString *)credentialDefJSON
                                          proverDID:(NSString *)proverDID
                                     masterSecretID:(NSString *)masterSecretID
                                       walletHandle:(IndyHandle)walletHandle
                                         completion:(void (^)(NSError *error, NSString *credReqJSON, NSString *credReqMetadataJSON))completion;

/**
 Check credential provided by Issuer for the given credential request,
 updates the credential by a master secret and stores in a secure wallet.
 
 @param credID: (optional, default is a random one) identifier by which credential will be stored in the wallet
 @param credReqJSON: a credential request created by proverCreateCredentialReqForCredentialOffer
 @param credReqMetadataJSON: a credential request metadata created by proverCreateCredentialReqForCredentialOffer
 @param credJson:  credential json received from issuer
 @param credDefJSON: credential definition json
 @param revRegDefJSON: revocation registry definition json
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter.
 */
+ (void)proverStoreCredential:(NSString *)credJson
                       credID:(NSString *)credID
                  credReqJSON:(NSString *)credReqJSON
          credReqMetadataJSON:(NSString *)credReqMetadataJSON
                  credDefJSON:(NSString *)credDefJSON
                revRegDefJSON:(NSString *)revRegDefJSON
                 walletHandle:(IndyHandle)walletHandle
                   completion:(void (^)(NSError *error, NSString *outCredID))completion;

/**
 Gets human readable credentials according to the filter.
 If filter is NULL, then all credentials are returned.
 Credentials can be filtered by Issuer, credential_def and/or Schema.
  
 @param filterJSON: filter for credentials
        {
            "schema_id": string, (Optional)
            "schema_issuer_did": string, (Optional)
            "schema_name": string, (Optional)
            "schema_version": string, (Optional)
            "issuer_did": string, (Optional)
            "cred_def_id": string, (Optional)
        }
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter. 
 Returns credentials json. 
  [{
      "referent": string, // cred_id in the wallet
      "values": <see credValuesJSON above>,
      "schema_id": string,
      "cred_def_id": string,
      "rev_reg_id": Optional<string>,
      "cred_rev_id": Optional<string>
  }]

 */
+ (void)proverGetCredentialsForFilter:(NSString *)filterJSON
                         walletHandle:(IndyHandle)walletHandle
                           completion:(void (^)(NSError *error, NSString *credentialsJSON))completion;

/**
 Gets human readable credentials matching the given proof request.
 
 @param  proofReqJSON: proof request json
    {
        "name": string,
        "version": string,
        "nonce": string,
        "requested_attributes": { // set of requested attributes
             "<attr_referent>": <attr_info>, // see below
             ...,
        },
        "requested_predicates": { // set of requested predicates
             "<predicate_referent>": <predicate_info>, // see below
             ...,
         },
        "non_revoked": Optional<<non_revoc_interval>>, // see below,
                       // If specified prover must proof non-revocation
                       // for date in this interval for each attribute
                       // (can be overridden on attribute level)
    }
 where 
 attr_referent: Proof-request local identifier of requested attribute
 attr_info: Describes requested attribute
    {
        "name": string, // attribute name, (case insensitive and ignore spaces)
        "restrictions": Optional<[<attr_filter>]> // see below,
                         // if specified, credential must satisfy to one of the given restriction.
        "non_revoked": Optional<<non_revoc_interval>>, // see below,
                       // If specified prover must proof non-revocation
                       // for date in this interval this attribute
                       // (overrides proof level interval)
    }
 predicate_referent: Proof-request local identifier of requested attribute predicate
 predicate_info: Describes requested attribute predicate
     {
         "name": attribute name, (case insensitive and ignore spaces)
         "p_type": predicate type (Currently >= only)
         "p_value": predicate value
         "restrictions": Optional<[<attr_filter>]> // see below,
                         // if specified, credential must satisfy to one of the given restriction.
         "non_revoked": Optional<<non_revoc_interval>>, // see below,
                        // If specified prover must proof non-revocation
                        // for date in this interval this attribute
                        // (overrides proof level interval)
     }
 non_revoc_interval: Defines non-revocation interval
     {
         "from": Optional<int>, // timestamp of interval beginning
         "to": Optional<int>, // timestamp of interval ending
     }
 filter: see filterJSON above      
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithNam).
 @param completion Callback that takes command result as parameter. Returns json with credentials for the given pool request.
     {
         "requested_attrs": {
             "<attr_referent>": [{ cred_info: <credential_info>, interval: Optional<non_revoc_interval> }],
             ...,
         },
         "requested_predicates": {
             "requested_predicates": [{ cred_info: <credential_info>, timestamp: Optional<integer> }, { cred_info: <credential_2_info>, timestamp: Optional<integer> }],
             "requested_predicate_2_referent": [{ cred_info: <credential_2_info>, timestamp: Optional<integer> }]
         }
     }, where credential is
     {
         "referent": <string>,
         "attrs": [{"attr_name" : "attr_raw_value"}],
         "schema_id": string,
         "cred_def_id": string,
         "rev_reg_id": Optional<int>,
         "cred_rev_id": Optional<int>,
     }
 */
+ (void)proverGetCredentialsForProofReq:(NSString *)proofReqJSON
                           walletHandle:(IndyHandle)walletHandle
                             completion:(void (^)(NSError *error, NSString *credentialsJSON))completion;

/**
 Creates a proof according to the given proof request
 Either a corresponding credential with optionally revealed attributes or self-attested attribute must be provided
 for each requested attribute (see indy_prover_get_credentials_for_pool_req).
 A proof request may request multiple credentials from different schemas and different issuers.
 All required schemas, public keys and revocation registries must be provided.
 The proof request also contains nonce.
 The proof contains either proof or self-attested attribute value for each requested attribute.
 
 @param  proofReqJSON: proof request json
    {
        "name": string,
        "version": string,
        "nonce": string,
        "requested_attributes": { // set of requested attributes
             "<attr_referent>": <attr_info>, // see below
             ...,
        },
        "requested_predicates": { // set of requested predicates
             "<predicate_referent>": <predicate_info>, // see below
             ...,
         },
        "non_revoked": Optional<<non_revoc_interval>>, // see below,
                       // If specified prover must proof non-revocation
                       // for date in this interval for each attribute
                       // (can be overridden on attribute level)
    }
 @param requestedCredentialsJSON: either a credential or self-attested attribute for each requested attribute
     {
         "self_attested_attributes": {
             "self_attested_attribute_referent": string
         },
         "requested_attributes": {
             "requested_attribute_referent_1": {"cred_id": string, "timestamp": Optional<number>, revealed: <bool> }},
             "requested_attribute_referent_2": {"cred_id": string, "timestamp": Optional<number>, revealed: <bool> }}
         },
         "requested_predicates": {
             "requested_predicates_referent_1": {"cred_id": string, "timestamp": Optional<number> }},
         }
     }
 @param masterSecretID: the id of the master secret stored in the wallet
 @param schemasJSON: all schemas json participating in the proof request
     {
         <schema1_id>: <schema1_json>,
         <schema2_id>: <schema2_json>,
         <schema3_id>: <schema3_json>,
     }
 @param credentialDefsJSON: all credential definitions json participating in the proof request
     {
         "cred_def1_id": <credential_def1_json>,
         "cred_def2_id": <credential_def2_json>,
         "cred_def3_id": <credential_def3_json>,
     }
 @param revocStatesJSON: all revocation states json participating in the proof request
     {
         "rev_reg_def1_id": {
             "timestamp1": <rev_state1>,
             "timestamp2": <rev_state2>,
         },
         "rev_reg_def2_id": {
             "timestamp3": <rev_state3>
         },
         "rev_reg_def3_id": {
             "timestamp4": <rev_state4>
         },
     }
 
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithNam).
 
 @param completion Callback that takes command result as parameter. 
  Proof json
  For each requested attribute either a proof (with optionally revealed attribute value) or
  self-attested attribute value is provided.
  Each proof is associated with a credential and corresponding schema_id, cred_def_id, rev_reg_id and timestamp.
  There is also aggregated proof part common for all credential proofs.
      {
          "requested": {
              "revealed_attrs": {
                  "requested_attr1_id": {sub_proof_index: number, raw: string, encoded: string},
                  "requested_attr4_id": {sub_proof_index: number: string, encoded: string},
              },
              "unrevealed_attrs": {
                  "requested_attr3_id": {sub_proof_index: number}
              },
              "self_attested_attrs": {
                  "requested_attr2_id": self_attested_value,
              },
              "requested_predicates": {
                  "requested_predicate_1_referent": {sub_proof_index: int},
                  "requested_predicate_2_referent": {sub_proof_index: int},
              }
          }
          "proof": {
              "proofs": [ <credential_proof>, <credential_proof>, <credential_proof> ],
              "aggregated_proof": <aggregated_proof>
          }
          "identifiers": [{schema_id, cred_def_id, Optional<rev_reg_id>, Optional<timestamp>}]
      }
  */
+ (void)proverCreateProofForRequest:(NSString *)proofRequestJSON
           requestedCredentialsJSON:(NSString *)requestedCredentialsJSON
                     masterSecretID:(NSString *)masterSecretID
                        schemasJSON:(NSString *)schemasJSON
                 credentialDefsJSON:(NSString *)credentialDefsJSON
                    revocStatesJSON:(NSString *)revocStatesJSON
                       walletHandle:(IndyHandle)walletHandle
                         completion:(void (^)(NSError *error, NSString *proofJSON))completion;

/**
 Verifies a proof (of multiple credential).
 All required schemas, public keys and revocation registries must be provided.
 
 @param  proofRequestJson: proof request json
    {
        "name": string,
        "version": string,
        "nonce": string,
        "requested_attributes": { // set of requested attributes
             "<attr_referent>": <attr_info>, // see below
             ...,
        },
        "requested_predicates": { // set of requested predicates
             "<predicate_referent>": <predicate_info>, // see below
             ...,
         },
        "non_revoked": Optional<<non_revoc_interval>>, // see below,
                       // If specified prover must proof non-revocation
                       // for date in this interval for each attribute
                       // (can be overridden on attribute level)
    }
 @param proofJSON: proof json
      {
          "requested": {
              "revealed_attrs": {
                  "requested_attr1_id": {sub_proof_index: number, raw: string, encoded: string},
                  "requested_attr4_id": {sub_proof_index: number: string, encoded: string},
              },
              "unrevealed_attrs": {
                  "requested_attr3_id": {sub_proof_index: number}
              },
              "self_attested_attrs": {
                  "requested_attr2_id": self_attested_value,
              },
              "requested_predicates": {
                  "requested_predicate_1_referent": {sub_proof_index: int},
                  "requested_predicate_2_referent": {sub_proof_index: int},
              }
          }
          "proof": {
              "proofs": [ <credential_proof>, <credential_proof>, <credential_proof> ],
              "aggregated_proof": <aggregated_proof>
          }
          "identifiers": [{schema_id, cred_def_id, Optional<rev_reg_id>, Optional<timestamp>}]
      }
 @param schemasJSON: all schemas json participating in the proof request
     {
         <schema1_id>: <schema1_json>,
         <schema2_id>: <schema2_json>,
         <schema3_id>: <schema3_json>,
     }
 @param credentialDefsJSON: all credential definitions json participating in the proof request
     {
         "cred_def1_id": <credential_def1_json>,
         "cred_def2_id": <credential_def2_json>,
         "cred_def3_id": <credential_def3_json>,
     }
 @param revocRegDefsJSON: all revocation registry definitions json participating in the proof
     {
         "rev_reg_def1_id": <rev_reg_def1_json>,
         "rev_reg_def2_id": <rev_reg_def2_json>,
         "rev_reg_def3_id": <rev_reg_def3_json>,
     }
 @param revocRegsJSON: all revocation registries json participating in the proof
     {
         "rev_reg_def1_id": {
             "timestamp1": <rev_reg1>,
             "timestamp2": <rev_reg2>,
         },
         "rev_reg_def2_id": {
             "timestamp3": <rev_reg3>
         },
         "rev_reg_def3_id": {
             "timestamp4": <rev_reg4>
         },
     }

 @param completion Callback that takes command result as parameter. Returns result flag: valid: true - if signature is valid, false - otherwise.

 */
+ (void)verifierVerifyProofRequest:(NSString *)proofRequestJson
                         proofJSON:(NSString *)proofJSON
                       schemasJSON:(NSString *)schemasJSON
                credentialDefsJSON:(NSString *)credentialDefsJSON
                  revocRegDefsJSON:(NSString *)revocRegDefsJSON
                     revocRegsJSON:(NSString *)revocRegsJSON
                        completion:(void (^)(NSError *error, BOOL valid))completion;

/**
 Create revocation state for a credential in the particular time moment.

 @param  credRevID: user credential revocation id in revocation registry
 @param  timestamp: time represented as a total number of seconds from Unix Epoch
 @param  revRegDefJSON: revocation registry definition json
 @param  revRegDeltaJSON: revocation registry definition delta json
 @param  blobStorageReaderHandle: configuration of blob storage reader handle that will allow to read revocation tails
 @param completion Callback that takes command result as parameter. 
 Returns result revocation state json:
 {
     "rev_reg": <revocation registry>,
     "witness": <witness>,
     "timestamp" : integer
 }
 */
+ (void)createRevocationStateForCredRevID:(NSString *)credRevID
                                timestamp:(NSNumber *)timestamp
                            revRegDefJSON:(NSString *)revRegDefJSON
                          revRegDeltaJSON:(NSString *)revRegDeltaJSON
                  blobStorageReaderHandle:(NSNumber *)blobStorageReaderHandle
                               completion:(void (^)(NSError *error, NSString *revStateJSON))completion;

/**
 Create new revocation state for a credential based on existed state at the particular time moment (to reduce calculation time).

 @param  revStateJSON: revocation registry state json
 @param  credRevID: user credential revocation id in revocation registry
 @param  timestamp: time represented as a total number of seconds from Unix Epoch
 @param  revRegDefJSON: revocation registry definition json
 @param  revRegDeltaJSON: revocation registry definition delta json
 @param  blobStorageReaderHandle: configuration of blob storage reader handle that will allow to read revocation tails
 @param completion Callback that takes command result as parameter.
 Returns result revocation state json:
 {
     "rev_reg": <revocation registry>,
     "witness": <witness>,
     "timestamp" : integer
 }
 */
+ (void)updateRevocationState:(NSString *)revStateJSON
                    credRevID:(NSString *)credRevID
                    timestamp:(NSNumber *)timestamp
                revRegDefJSON:(NSString *)revRegDefJSON
              revRegDeltaJSON:(NSString *)revRegDeltaJSON
      blobStorageReaderHandle:(NSNumber *)blobStorageReaderHandle
                   completion:(void (^)(NSError *error, NSString *updatedRevStateJSON))completion;

@end
