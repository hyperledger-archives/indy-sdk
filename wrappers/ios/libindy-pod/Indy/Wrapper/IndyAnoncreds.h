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
 
 @param issuerDID DID of the issuer signing credential_def transaction to the Ledger
 @param schemaJSON Schema as a json
 @param tag: allows to distinct between credential definitions for the same issuer and schema
 @param type: type_: credential definition type (optional, 'CL' by default) that defines claims signature and revocation math. 
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
 @param tailsWriterType:
 @param tailsWriterConfig:
 @param completion Callback that takes command result as parameter.
 Returns revocation registry definition json and revocation registry entry json.
 */
+ (void)issuerCreateAndStoreRevocRegForCredentialDefId:(NSString *)credDefID
                                             issuerDID:(NSString *)issuerDID
                                                  type:(NSString *)type
                                                   tag:(NSString *)tag
                                            configJSON:(NSString *)configJSON
                                       tailsWriterType:(NSString *)tailsWriterType
                                     tailsWriterConfig:(NSString *)tailsWriterConfig
                                          walletHandle:(IndyHandle)walletHandle
                                            completion:(void (^)(NSError *error, NSString *revocRegID, NSString *revocRegDefJSON, NSString *revocRegEntryJSON))completion;

/**
 Create credential offer and store it in wallet.

 @param cred_def_id: id of stored in ledger credential definition
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter.
 Returns credentials offer json
        {
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
 @param  credRequestJSON: a credential request created by proverCreateCredentialReqWithCredentialDef
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
     credJSON: Contains signed credential values
     {
         "cred_def_id": string,
         "rev_reg_id", Optional<string>,
         "values": <see credential_values_json above>,
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
 Revoke a credential identified by a cred_revoc_id (returned by indy_issuer_create_cred).

 The corresponding credential definition and revocation registry must be already
 created an stored into the wallet.

 This call returns revoc registry delta as json file intended to be shared as REVOC_REG_ENTRY transaction.
 Note that it is possible to accumulate deltas to reduce ledger load.

 @param revRegId: id of revocation registry stored in wallet
 @param blobStorageReaderHandle: pre-configured blob storage reader instance handle that will allow to read revocation tails
 @param credRevocId: local id for revocation info
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter. 
 Revocation registry delta json with a revoked credential
 */
+ (void)issuerRevokeCredentialByCredRevocId:(NSString *)credRevocId
                                   revRegId:(NSString *)revRegId
                    blobStorageReaderHandle:(NSNumber *)blobStorageReaderHandle
                               walletHandle:(IndyHandle)walletHandle
                                 completion:(void (^)(NSError *error, NSString *revocRegDeltaJSON))completion;

+ (void)issuerRecoverCredentialByCredRevocId:(NSString *)credRevocId
                                    revRegId:(NSString *)revRegId
                     blobStorageReaderHandle:(NSNumber *)blobStorageReaderHandle
                                walletHandle:(IndyHandle)walletHandle
                                  completion:(void (^)(NSError *error, NSString *revocRegDeltaJSON))completion;

/**
 Creates a master secret with a given name and stores it in the wallet.
 The name must be unique.
 
 @param masterSecretID A new master secret name.
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Returns error code.
 
 */
+ (void)proverCreateMasterSecret:(NSString *)masterSecretID
                    walletHandle:(IndyHandle)walletHandle
                      completion:(void (^)(NSError *error))completion;

/**
 
 Creates a clam request for the given credential offer.

 The method creates a blinded master secret for a master secret identified by a provided name.
 The master secret identified by the name must be already stored in the secure wallet (see prover_create_master_secret)
 The blinded master secret is a part of the credential request.

 @param proverDID: a DID of the prover
 @param credOfferJSON: a cred offer created by issuerCreateCredentialOfferForCredDefId
 @param credentialDefJSON: credential definition json created by issuerCreateAndStoreCredentialDefForIssuerDID
 @param masterSecretID: the name of the master secret stored in the wallet
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter. 
 Returns 
     credReqJSON: Credential request json for creation of credential by Issuer
     {
      "cred_def_id" : string,
      "rev_reg_id" : Optional<string>,
      "prover_did" : string,
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
 
 @param credID: identifier by which credential will be stored in wallet
 @param credReqJSON: a credential request created by proverCreateCredentialReqForCredentialOffer
 @param credReqMetadataJSON: a credential request metadata created by proverCreateCredentialReqForCredentialOffer
 @param credJson: credential json created by issuerCreateCredentialForCredOffer
 @param credDefJSON: credential definition json created by issuerCreateSchemaForIssuerDID
 @param revRegDefJSON: revocation registry definition json
 @param revStateJSON: revocation state json
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter.
 */
+ (void)proverStoreCredential:(NSString *)credJson
                       credID:(NSString *)credID
                  credReqJSON:(NSString *)credReqJSON
          credReqMetadataJSON:(NSString *)credReqMetadataJSON
                  credDefJSON:(NSString *)credDefJSON
                revRegDefJSON:(NSString *)revRegDefJSON
                 revStateJSON:(NSString *)revStateJSON
                 walletHandle:(IndyHandle)walletHandle
                   completion:(void (^)(NSError *error, NSString *outCredID))completion;

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
 @param masterSecretID: the name of the master secret stored in the wallet
 @param credentialDefsJSON: all credential definition jsons participating in the proof request
     {
         "credential1_referent_in_wallet": <credential_def1>,
         "credential2_referent_in_wallet": <credential_def2>,
         "credential3_referent_in_wallet": <credential_def3>,
     }
 @param revocStatesJSON: all revocation registry jsons participating in the proof request
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
                     masterSecretID:(NSString *)masterSecretID
                        schemasJSON:(NSString *)schemasJSON
                 credentialDefsJSON:(NSString *)credentialDefsJSON
                    revocStatesJSON:(NSString *)revocStatesJSON
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

+ (void)createRevocationStateForCredRevID:(NSString *)credRevID
                                timestamp:(NSNumber *)timestamp
                            revRegDefJSON:(NSString *)revRegDefJSON
                          revRegDeltaJSON:(NSString *)revRegDeltaJSON
                  blobStorageReaderHandle:(NSNumber *)blobStorageReaderHandle
                               completion:(void (^)(NSError *error, NSString *revStateJSON))completion;

+ (void)updateRevocationState:(NSString *)revStateJSON
                    credRevID:(NSString *)credRevID
                    timestamp:(NSNumber *)timestamp
                revRegDefJSON:(NSString *)revRegDefJSON
              revRegDeltaJSON:(NSString *)revRegDeltaJSON
      blobStorageReaderHandle:(NSNumber *)blobStorageReaderHandle
                   completion:(void (^)(NSError *error, NSString *updatedRevStateJSON))completion;

@end
