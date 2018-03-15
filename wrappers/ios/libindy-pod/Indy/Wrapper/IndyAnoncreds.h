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
 
 @param issuerDID DID of the issuer signing credential_def transaction to the Ledger
 @param name a name the schema
 @param version a version of the schema
 @param attrs a list of schema attributes descriptions
 @param completion Callback that takes command result as parameter. Returns schemaId and schemaJson.
*/
+ (void)issuerCreateSchemaForIssuerDID:(NSString *)issuerDID
                                  name:(NSString *)name
                               version:(NSString *)version
                                 attrs:(NSString *)attrs
                            completion:(void (^)(NSError *error, NSString *schemaId, NSString *schemaJSON))completion;

/**
 Creates keys (both primary and revocation) for the given schema and signature type (currently only CL signature type is supported).
 Stores the keys together with signature type and schema in a secure wallet as a credential definition.
 
 The credential definition in the wallet is identifying by a returned unique key.
 
 @param issuerDID DID of the issuer signing credential_def transaction to the Ledger
 @param schemaJSON Schema as a json
 @param tag:
 @param type: (optional) signature type. Currently only 'CL' is supported.
 @param configJSON: config json.
     {
         "support_revocation": boolean
     }
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter. 
 Returns credential definition json containing information about signature type, schema and issuer's public key.
*/
+ (void)issuerCreateAndStoreCredentialDefForIssuerDID:(NSString *)issuerDID
                                           schemaJSON:(NSString *)schemaJSON
                                                  tag:(NSString *)tag
                                                 type:(NSString *)type
                                           configJSON:(NSString *)configJSON
                                         walletHandle:(IndyHandle)walletHandle
                                           completion:(void (^)(NSError *error, NSString *credentialDefId, NSString *credentialDefJSON))completion;

/**
 Creates a new revocation registry for the given credential definition.
 Stores it in a secure wallet identifying by the returned key.
 
 @param walletHandle: wallet handler (created by open_wallet).
 @param issuerDID: a DID of the issuer signing transaction to the Ledger
 @param type: (optional) registry type. Currently only 'CL_ACCUM' is supported.
 @param tag:
 @param credDefId: id of stored in ledger credential definition
 @param configJSON: {
     "issuance_type": (optional) type of issuance. Currently supported:
         1) ISSUANCE_BY_DEFAULT: all indices are assumed to be issued and initial accumulator is calculated over all indices;
                                 Revocation Registry is updated only during revocation.
         2) ISSUANCE_ON_DEMAND: nothing is issued initially accumulator is 1 (used by default);
     "max_cred_num": maximum number of credentials the new registry can process.
 }
 @param tailsWriterType:
 @param tailsWriterConfig:
 @param completion Callback that takes command result as parameter.
 Returns revocation registry definition json and revocation registry entry json.
 */
+ (void)issuerCreateAndStoreRevocRegForIssuerDid:(NSString *)issuerDID
                                            type:(NSString *)type
                                             tag:(NSString *)tag
                                       credDefId:(NSString *)credDefId
                                      configJSON:(NSString *)configJSON
                                 tailsWriterType:(NSString *)tailsWriterType
                               tailsWriterConfig:(NSString *)tailsWriterConfig
                                    walletHandle:(IndyHandle)walletHandle
                                      completion:(void (^)(NSError *error, NSString *revocRegID, NSString *revocRegDefJSON, NSString *revocRegEntryJSON))completion;

/**
 Create credential offer and store it in wallet.

 @param cred_def_id: id of stored in ledger credential definition
 @param issuer_did: a DID of the issuer of credential
 @param prover_did: a DID of the target user
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter.
 Returns credentials offer json
        {
            "cred_def_id": string,
            "issuer_did" : string,
            "nonce": string,
            "key_correctness_proof" : <key_correctness_proof>
        }
*/
+ (void)issuerCreateCredentialOfferForProverDID:(NSString *)proverDID
                                      issuerDID:(NSString *)issuerDID
                                      credDefId:(NSString *)credDefId
                                   walletHandle:(IndyHandle)walletHandle
                                     completion:(void (^)(NSError *error, NSString *credentialOfferJSON))completion;

/**
 Signs a given credential values for the given user by a given key (credential def).
 The corresponding credential definition and revocation registry must be already created
 an stored into the wallet.

 @param  credentialRequestJSON: a credential request with a blinded secret from the user (returned by prover_create_and_store_credential_req).
     Example:
     {
      "blinded_ms" : <blinded_master_secret>,
      "cred_def_id" : string,
      "issuer_did" : string,
      "prover_did" : string,
      "blinded_ms_correctness_proof" : <blinded_ms_correctness_proof>,
      "nonce": string
    }
 @param  credentialValuesJSON: a credential containing attribute values for each of requested attribute names.
     Example:
     {
      "attr1" : {"raw": "value1", "encoded": "value1_as_int" },
      "attr2" : {"raw": "value1", "encoded": "value1_as_int" }
     }
 @param revRegId: (Optional) id of stored in ledger revocation registry definition
 @param tailsReaderHandle:
 @param userRevocIndex: index of a new user in the revocation registry (optional, pass -1 if user_revoc_index is absentee; default one is used if not provided)
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter. 
 Revocation registry update json with a newly issued credential
 Credential json containing signed credential values, issuer_did, schema_key, and revoc_reg_seq_no
 used for issuance
     {
         "values": <see credential_values_json above>,
         "signature": <signature>,
         "issuer_did": string,
         "cred_def_id": string,
         "rev_reg_id", Optional<string>,
         "signature_correctness_proof": <signature_correctness_proof>
     }
 */
+ (void)issuerCreateCredentialWithRequest:(NSString *)credentialRequestJSON
                     credentialValuesJSON:(NSString *)credentialValuesJSON
                                 revRegId:(NSString *)revRegId
                        tailsReaderHandle:(NSNumber *)tailsReaderHandle
                           userRevocIndex:(NSNumber *)userRevocIndex
                             walletHandle:(IndyHandle)walletHandle
                               completion:(void (^)(NSError *error, NSString *revocRegDeltaJSON, NSString *xcredentialJSON))completion;

/**
 Revokes a user identified by a user_revoc_index in a given revoc-registry.
 The corresponding credential definition and revocation registry must be already
 created an stored into the wallet.
 
 @param rev_reg_id: id of revocation registry stored in wallet
 @param tails_reader_handle:
 @param user_revoc_index: index of the user in the revocation registry
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter. 
 Revocation registry delta json with a revoked credential
 */
+ (void)issuerRevokeCredentialForRevRegId:(NSString *)revRegId
                        tailsReaderHandle:(NSNumber *)tailsReaderHandle
                           userRevocIndex:(NSNumber *)userRevocIndex
                             walletHandle:(IndyHandle)walletHandle
                               completion:(void (^)(NSError *error, NSString *revocRegDeltaJSON))completion;

+ (void)issuerRecoverCredentialForRevRegId:(NSString *)revRegId
                         tailsReaderHandle:(NSNumber *)tailsReaderHandle
                            userRevocIndex:(NSNumber *)userRevocIndex
                              walletHandle:(IndyHandle)walletHandle
                                completion:(void (^)(NSError *error, NSString *revocRegDeltaJSON))completion;

/**
 Stores a credential offer from the given issuer in a secure storage.
  
 @param credentialOfferJSON: credential offer as a json containing information about the issuer and a credential:
        {
            "cred_def_id": string,
            "rev_reg_id" : Optional<string>,
            "nonce": string,
            "key_correctness_proof" : <key_correctness_proof>
        } 
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 */
+ (void)proverStoreCredentialOffer:(NSString *)credentialOfferJSON
                  WithWalletHandle:(IndyHandle)walletHandle
                        completion:(void (^)(NSError *error))completion;

/**
 Gets all stored credential offers (see prover_store_credential_offer).
 A filter can be specified to get credential offers for specific Issuer, credential_def or schema only.
 
 @param filterJSON: optional filter to get credential offers for specific Issuer, credential_def or schema only only
     Each of the filters is optional and can be combines
        {
            "schema_id": string, (Optional)
            "schema_did": string, (Optional)
            "schema_name": string, (Optional)
            "schema_version": string, (Optional)
            "issuer_did": string, (Optional)
            "issuer_did": string, (Optional)
            "cred_def_id": string, (Optional)
        }
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Returns A json with a list of credentials offers for the filter.
 */
+ (void)proverGetCredentialOffersWithFilter:(NSString *)filterJSON
                               walletHandle:(IndyHandle)walletHandle
                                 completion:(void (^)(NSError *error, NSString *credentialOffersJSON))completion;


/**
 Creates a master secret with a given name and stores it in the wallet.
 The name must be unique.
 
 @param masterSecretName A new master secret name.
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Returns error code.
 
 */
+ (void)proverCreateMasterSecretNamed:(NSString *)masterSecretName
                         walletHandle:(IndyHandle)walletHandle
                           completion:(void (^)(NSError *error))completion;

/**
 
 Creates a clam request json for the given credential offer and stores it in a secure wallet.
 The credential offer contains the information about Issuer (DID, schema_seq_no),
 and the schema (schema_key).
 The method creates a blinded master secret for a master secret identified by a provided name.
 The master secret identified by the name must be already stored in the secure wallet (see prover_create_master_secret)
 The blinded master secret is a part of the credential request.
  
 @param proverDID: a DID of the prover
 @param credentialOfferJSON: credential offer as a json containing information about the issuer and a credential:
        {
            "cred_def_id": string,
            "rev_reg_id" : Optional<string>,
            "nonce": string,
            "key_correctness_proof" : <key_correctness_proof>
        }
 @param credentialDefJSON: credential definition json associated with issuer_did and schema_seq_no in the credential_offer
 @param masterSecretName: the name of the master secret stored in the wallet
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter. Returns Credential request json.
 */
+ (void)proverCreateAndStoreCredentialReqWithCredentialDef:(NSString *)credentialDefJSON
                                                 proverDID:(NSString *)proverDID
                                       credentialOfferJSON:(NSString *)credentialOfferJSON
                                          masterSecretName:(NSString *)masterSecretName
                                              walletHandle:(IndyHandle)walletHandle
                                                completion:(void (^)(NSError *error, NSString *credentialReqJSON))completion;

/**
 Updates the credential by a master secret and stores in a secure wallet.
 The credential contains the information about
 schema_key, issuer_did, revoc_reg_seq_no (see issuer_create_credential).
 Seq_no is a sequence number of the corresponding transaction in the ledger.
 The method loads a blinded secret for this key from the wallet,
 updates the credential and stores it in a wallet.
 
 @param credentialId: identifier by which credential will be stored in wallet
 @param credentialsJson: credential json:
     {
         "values": <see credential_values_json above>,
         "signature": <signature>,
         "cred_def_id": string,
         "rev_reg_id", Optional<string>,
         "signature_correctness_proof": <signature_correctness_proof>
     }
 @param revRegDefJSON: revocation registry definition json
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter.
 */
+ (void)proverStoreCredential:(NSString *)credentialsJson
                 credentialId:(NSString *)credentialId
                revRegDefJSON:(NSString *)revRegDefJSON
                 walletHandle:(IndyHandle)walletHandle
                   completion:(void (^)(NSError *error))completion;

/**
 Gets human readable credentials according to the filter.
 If filter is NULL, then all credentials are returned.
 Credentials can be filtered by Issuer, credential_def and/or Schema.
  
 @param filterJSON: filter for credentials
        {
            "schema_id": string, (Optional)
            "schema_did": string, (Optional)
            "schema_name": string, (Optional)
            "schema_version": string, (Optional)
            "issuer_did": string, (Optional)
            "issuer_did": string, (Optional)
            "cred_def_id": string, (Optional)
        }
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter. 
 Returns credentials json. See example above.

 */
+ (void)proverGetCredentialsWithFilter:(NSString *)filterJSON
                          walletHandle:(IndyHandle)walletHandle
                            completion:(void (^)(NSError *error, NSString *credentialsJSON))completion;

/**
 Gets human readable credentials matching the given proof request.
 
 @param  proofReqJSON: proof request json
     {
         "name": string,
         "version": string,
         "nonce": string,
         "requested_attrs": {
             "requested_attr1_referent": <attr_info>,
             "requested_attr2_referent": <attr_info>,
             "requested_attr3_referent": <attr_info>,
         },
         "requested_predicates": {
             "requested_predicate_1_referent": <predicate_info>,
             "requested_predicate_2_referent": <predicate_info>,
         },
         "freshness": Optional<number>
     }

 where attr_info:
     {
         "name": attribute name, (case insensitive and ignore spaces)
         "freshness": (Optional)
         "restrictions": [
             <see filter json above>
         ]  (Optional) - if specified, credential must satisfy to one of the given restriction.
     }
 predicate_info:
     {
         "attr_name": attribute name, (case insensitive and ignore spaces)
         "p_type": predicate type (Currently >= only)
         "value": requested value of attribute
         "freshness": (Optional)
         "restrictions": [
             <see filter json above>
         ]  (Optional) - if specified, credential must satisfy to one of the given restriction.
     }
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithNam).
 @param completion Callback that takes command result as parameter. Returns json with credentials for the given pool request. 
 json with credentials for the given pool request.
     {
         "attrs": {
             "requested_attr1_referent": [(credential1, Optional<freshness>), (credential2, Optional<freshness>)],
             "requested_attr2_referent": [],
             "requested_attr3_referent": [(credential3, Optional<freshness>)]
         },
         "predicates": {
             "requested_predicate_1_referent": [(credential1, Optional<freshness>), (credential3, Optional<freshness>)],
             "requested_predicate_2_referent": [(credential2, Optional<freshness>)]
         }
     }, where credential is
     {
         "referent": <string>,
         "attrs": [{"attr_name" : "attr_raw_value"}],
         "issuer_did": string,
         "cred_def_id": string,
         "rev_reg_id": Optional<int>
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
 
 @param proofRequestJSON: proof request json as come from the verifier
     {
         "name": string,
         "version": string,
         "nonce": string,
         "requested_attrs": {
             "requested_attr1_referent": <attr_info>,
             "requested_attr2_referent": <attr_info>,
             "requested_attr3_referent": <attr_info>,
         },
         "requested_predicates": {
             "requested_predicate_1_referent": <predicate_info>,
             "requested_predicate_2_referent": <predicate_info>,
         },
         "freshness": Optional<number>
     }
 @param requestedCredentialsJSON: either a credential or self-attested attribute for each requested attribute
     {
         "requested_attr1_referent": [{"cred_id": string, "freshness": Optional<number>}, true <reveal_attr>],
         "requested_attr2_referent": [self_attested_attribute],
         "requested_attr3_referent": [{"cred_id": string, "freshness": Optional<number>}, false]
         "requested_attr4_referent": [{"cred_id": string, "freshness": Optional<number>}, true]
         "requested_predicate_1_referent": [{"cred_id": string, "freshness": Optional<number>}],
         "requested_predicate_2_referent": [{"cred_id": string, "freshness": Optional<number>}],
     }
 @param schemasJSON: all schema jsons participating in the proof request
     {
         "credential1_referent_in_wallet": <schema1>,
         "credential2_referent_in_wallet": <schema2>,
         "credential3_referent_in_wallet": <schema3>,
     }
 @param masterSecretName: the name of the master secret stored in the wallet
 @param credentialDefsJSON: all credential definition jsons participating in the proof request
     {
         "credential1_referent_in_wallet": <credential_def1>,
         "credential2_referent_in_wallet": <credential_def2>,
         "credential3_referent_in_wallet": <credential_def3>,
     }
 @param revocInfosJSON: all revocation registry jsons participating in the proof request
     {
         "credential1_referent_in_wallet": {
             "freshness1": <revoc_info1>,
             "freshness2": <revoc_info2>,
         },
         "credential2_referent_in_wallet": {
             "freshness3": <revoc_info3>
         },
         "credential3_referent_in_wallet": {
             "freshness4": <revoc_info4>
         },
     }
 
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithNam).
 
 @param completion Callback that takes command result as parameter. 
 Proof json
 For each requested attribute either a proof (with optionally revealed attribute value) or
 self-attested attribute value is provided.
 Each proof is associated with a credential and corresponding schema_seq_no, issuer_did and revoc_reg_seq_no.
 There ais also aggregated proof part common for all credential proofs.
     {
         "requested": {
             "revealed_attrs": {
                 "requested_attr1_id": {referent: string, raw: string, encoded: string},
                 "requested_attr4_id": {referent: string, raw: string, encoded: string},
             },
             "unrevealed_attrs": {
                 "requested_attr3_id": referent
             },
             "self_attested_attrs": {
                 "requested_attr2_id": self_attested_value,
             },
             "requested_predicates": {
                 "requested_predicate_1_referent": [credential_proof2_referent],
                 "requested_predicate_2_referent": [credential_proof3_referent],
             }
         }
         "proof": {
             "proofs": {
                 "credential_proof1_referent": <credential_proof>,
                 "credential_proof2_referent": <credential_proof>,
                 "credential_proof3_referent": <credential_proof>
             },
             "aggregated_proof": <aggregated_proof>
         }
         "identifiers": {"credential_proof1_referent":{schema_id, cred_def_id, Optional<rev_reg_id>, Optional<timestamp>}}
     } */
+ (void)proverCreateProofForRequest:(NSString *)proofRequestJSON
           requestedCredentialsJSON:(NSString *)requestedCredentialsJSON
                        schemasJSON:(NSString *)schemasJSON
                   masterSecretName:(NSString *)masterSecretName
                 credentialDefsJSON:(NSString *)credentialDefsJSON
                     revocInfosJSON:(NSString *)revocInfosJSON
                       walletHandle:(IndyHandle)walletHandle
                         completion:(void (^)(NSError *error, NSString *proofJSON))completion;

/**
 Verifies a proof (of multiple credential).
 All required schemas, public keys and revocation registries must be provided.
 
 @param proofRequestJson: initial proof request as sent by the verifier
     {
         "name": string,
         "version": string,
         "nonce": string,
         "requested_attrs": {
             "requested_attr1_referent": <attr_info>,
             "requested_attr2_referent": <attr_info>,
             "requested_attr3_referent": <attr_info>,
         },
         "requested_predicates": {
             "requested_predicate_1_referent": <predicate_info>,
             "requested_predicate_2_referent": <predicate_info>,
         },
         "freshness": Optional<number>
     }
 @param proofJSON: proof json
 For each requested attribute either a proof (with optionally revealed attribute value) or
 self-attested attribute value is provided.
 Each proof is associated with a credential and corresponding schema_seq_no, issuer_did and revoc_reg_seq_no.
 There ais also aggregated proof part common for all credential proofs.
     {
         "requested": {
             "requested_attr1_id": [credential_proof1_referent, revealed_attr1, revealed_attr1_as_int],
             "requested_attr2_id": [self_attested_attribute],
             "requested_attr3_id": [credential_proof2_referent]
             "requested_attr4_id": [credential_proof2_referent, revealed_attr4, revealed_attr4_as_int],
             "requested_predicate_1_referent": [credential_proof2_referent],
             "requested_predicate_2_referent": [credential_proof3_referent],
         }
         "proof": {
             "proofs": {
                 "credential_proof1_referent": <credential_proof>,
                 "credential_proof2_referent": <credential_proof>,
                 "credential_proof3_referent": <credential_proof>
             },
             "aggregated_proof": <aggregated_proof>
         }
         "identifiers": {"credential_proof1_referent":{schema_id, cred_def_id, Optional<rev_reg_id>, Optional<timestamp>}}
     }
 @param schemasJSON: all schemas json participating in the proof
         {
             "credential_proof1_referent": <schema>,
             "credential_proof2_referent": <schema>,
             "credential_proof3_referent": <schema>
         }
 @param credentialDefsJSON: all credential definitions json participating in the proof
         {
             "credential_proof1_referent": <credential_def>,
             "credential_proof2_referent": <credential_def>,
             "credential_proof3_referent": <credential_def>
         }
 @param revocRegDefsJSON: all revocation registry definitions json participating in the proof
         {
             "credential_proof1_referent": <rev_reg_def>,
             "credential_proof2_referent": <rev_reg_def>,
             "credential_proof3_referent": <rev_reg_def>
         }
 @param revocRegsJSON: all revocation registry definitions json participating in the proof
     {
         "credential1_referent_in_wallet": {
             "freshness1": <revoc_reg1>,
             "freshness2": <revoc_reg2>,
         },
         "credential2_referent_in_wallet": {
             "freshness3": <revoc_reg3>
         },
         "credential3_referent_in_wallet": {
             "freshness4": <revoc_reg4>
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

+ (void)createRevocationInfoForTimestamp:(NSNumber *)timestamp
                           revRegDefJSON:(NSString *)revRegDefJSON
                         revRegDeltaJSON:(NSString *)revRegDeltaJSON
                       tailsReaderHandle:(NSNumber *)tailsReaderHandle
                                  revIdx:(NSNumber *)revIdx
                              completion:(void (^)(NSError *error, NSString *revInfo))completion;

+ (void)updateRevocationInfoForTimestamp:(NSNumber *)timestamp
                             revInfoJSON:(NSString *)revInfoJSON
                           revRegDefJSON:(NSString *)revRegDefJSON
                         revRegDeltaJSON:(NSString *)revRegDeltaJSON
                       tailsReaderHandle:(NSNumber *)tailsReaderHandle
                                  revIdx:(NSNumber *)revIdx
                              completion:(void (^)(NSError *error, NSString *updatedRevInfo))completion;

+ (void)storeRevocationInfoForId:(NSString *)id
                     revInfoJSON:(NSString *)revInfoJSON
                    walletHandle:(IndyHandle)walletHandle
                      completion:(void (^)(NSError *error))completion;

+ (void)getRevocationInfoForId:(NSString *)id
                     timestamp:(NSNumber *)timestamp
                  walletHandle:(IndyHandle)walletHandle
                    completion:(void (^)(NSError *error, NSString *revInfo))completion;

@end
