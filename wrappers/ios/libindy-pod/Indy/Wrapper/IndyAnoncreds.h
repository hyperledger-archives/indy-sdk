//
//  IndyAnoncreds.h
//  libindy
//


#import <Foundation/Foundation.h>
#import "IndyTypes.h"

@interface IndyAnoncreds : NSObject

/**
 Creates keys (both primary and revocation) for the given schema and signature type (currently only CL signature type is supported).
 Stores the keys together with signature type and schema in a secure wallet as a claim definition.
 
 The claim definition in the wallet is identifying by a returned unique key.
 
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param issuerDid DID of the issuer signing claim_def transaction to the Ledger
 @param schema Schema as a json
 @param signatureType Signature type (optional). Currently only 'CL' is supported.
 @param createNonRevoc Whether to request non-revocation claim.
 @param handler Callback that takes command result as parameter. Returns claim definition json containing information about signature type, schema and issuer's public key. Unique number identifying the public key in the wallet.

 @return Error Code.
*/
+ (NSError *)issuerCreateAndStoreClaimDefWithWalletHandle:(IndyHandle)walletHandle
                                                issuerDid:(NSString *)issuerDid
                                               schemaJSON:(NSString *)schema
                                            signatureType:(NSString *)signatureType
                                           createNonRevoc:(BOOL)createNonRevoc
                                               completion:(void (^)(NSError *error, NSString *claimDefJSON)) handler;

/**
 Creates a new revocation registry for the given claim definition.
 Stores it in a secure wallet identifying by the returned key.
 
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param issuerDid DID of the issuer signing revoc_reg transaction to the Ledger
 @param schemaSeqNo Seq no of a schema transaction in Ledger
 @param maxClaimNum Maximum number of claims the new registry can process.
 @param handler Callback that takes command result as parameter. Returns revoc registry json and unique number identifying the revocation registry in the wallet.
 
 @return Error Code
 */
+ (NSError *)issuerCreateAndStoreRevocRegWithWalletHandle:(IndyHandle)walletHandle
                                                issuerDid:(NSString *)issuerDid
                                              schemaSeqNo:(NSNumber *)schemaSeqNo
                                              maxClaimNum:(NSNumber *)maxClaimNum
                                               completion:(void (^)(NSError *error, NSString *revocRegJSON, NSString *revocRegUUID)) handler;

/**
 Signs a given claim for the given user by a given key (claim ef).
 The corresponding claim definition and revocation registry must be already created
 an stored into the wallet.

 @code
 Example claimReqJSON:
 {
 "blinded_ms" : <blinded_master_secret>,
 "schema_seq_no" : <schema_seq_no>,
 "issuer_did" : <issuer_did>
 }
 @endcode
 
 @code
 Example claimJSON:
 {
 "attr1" : ["value1", "value1_as_int"],
 "attr2" : ["value2", "value2_as_int"]
 }
 @endcode
 
 @code
 
 Example xclaimJSON:
 {
 "claim": <see claim_json above>,
 "signature": <signature>,
 "revoc_reg_seq_no", string,
 "issuer_did", string,
 "schema_seq_no", string,
 }
 
 @endcode

 
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param claimReqJSON Claim request with a blinded secret from the user (returned by IndyAnoncreds::proverCreateAndStoreClaimReqWithWalletHandle).
        Also contains schema_seq_no and issuer_did.
 
 @param claimJSON Claim containing attribute values for each of requested attribute names.
 
 @param userRevocIndex Index of a new user in the revocation registry (optional, pass -1 if user_revoc_index is absentee; default one is used if not provided)
 @param handler Callback that takes command result as parameter. Returns revocation registry update json with a newly issued claim and claim json containing issued claim, issuer_did, schema_seq_no, and revoc_reg_seq_no
 used for issuance.

 @return Error Code.
 */
+ (NSError *)issuerCreateClaimWithWalletHandle:(IndyHandle)walletHandle
                                  claimReqJSON:(NSString *)claimReqJSON
                                     claimJSON:(NSString *)claimJSON
                                userRevocIndex:(NSNumber *)userRevocIndex
                                    completion:(void (^)(NSError *error, NSString *revocRegUpdateJSON, NSString *xclaimJSON)) handler;

/**
 Revokes a user identified by a revoc_id in a given revoc-registry.
 The corresponding claim definition and revocation registry must be already
 created an stored into the wallet.
 
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param issuerDid DID of the issuer signing claim_def transaction to the Ledger.
 @param schemaSeqNo Seq no of a schema transaction in Ledger.
 @param userRevocIndex Index of the user in the revocation registry.
 @param handler Callback that takes command result as parameter. Returns revocation registry update json with a revoked claim.
 
 @return Error Code
 */
+ (NSError *)issuerRevokeClaimWithWalletHandle:(IndyHandle)walletHandle
                                     issuerDid:(NSString *)issuerDid
                                   schemaSeqNo:(NSNumber *)schemaSeqNo
                                userRevocIndex:(NSNumber *)userRevocIndex
                                    completion:(void (^)(NSError *error, NSString *revocRegUpdateJSON)) handler;

/**
 Stores a claim offer from the given issuer in a secure storage.
 
 @code
 Example claimOfferJSON:
 {
    "issuer_did": string,
    "schema_seq_no": string
 }
 @endcode
 
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param claimOfferJSON Claim offer as a json containing information about the issuer and a claim.

 @return Error Code.
 */
+ (NSError *)proverStoreClaimOfferWithWalletHandle:(IndyHandle)walletHandle
                                    claimOfferJSON:(NSString *)claimOfferJSON
                                        completion:(void (^)(NSError *error)) handler;

/**
 Gets all stored claim offers (see IndyAnoncreds::proverStoreClaimOfferWithWalletHandle).
 A filter can be specified to get claim offers for specific Issuer, claim_def or schema only.
 
 @code
 Example filterJSON:
 {
 "issuer_did": string,
 "schema_seq_no": string
 }
 @endcode
 
 @code
 Example claimOffersJSON:
 {
 [{"issuer_did": string,
 "schema_seq_no": string}]
 }
 @endcode

 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param filterJSON Optional filter to get claim offers for specific Issuer, claim_def or schema only only
 Each of the filters is optional.
 @param handler Returns A json with a list of claim offers for the filter.

 @return Error Code
 */
+ (NSError *)proverGetClaimOffersWithWalletHandle:(IndyHandle)walletHandle
                                       filterJSON:(NSString *)filterJSON
                                       completion:(void (^)(NSError *error, NSString *claimOffersJSON)) handler;


/**
 Creates a master secret with a given name and stores it in the wallet.
 The name must be unique.
 
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param masterSecretName A new master secret name.
 @param handler Returns error code.
 
 @return Error Code
 */
+ (NSError *)proverCreateMasterSecretWithWalletHandle:(IndyHandle)walletHandle
                                     masterSecretName:(NSString *)masterSecretName
                                           completion:(void (^)(NSError *error)) handler;

/**
 
 Creates a clam request json for the given claim offer and stores it in a secure wallet.
 
 The claim offer contains the information about Issuer (DID, schema_seq_no),
 and the schema (schema_seq_no).  
 
 The method gets public key and schema from the ledger, stores them in a wallet,
 and creates a blinded master secret for a master secret identified by a provided name.  
 
 The master secret identified by the name must be already stored in the secure wallet (see IndyAnoncreds::proverCreateMasterSecretWithWalletHandle)
 
 The blinded master secret is a part of the claim request.
 
 @code
 Example claimOfferJSON:
 {
 "issuer_did": string,
 "schema_seq_no": string
 }
 @endcode
 
 @code
 Example claimReqJSON returned in handle:
 {
 "blinded_ms" : <blinded_master_secret>,
 "schema_seq_no" : <schema_seq_no>,
 "issuer_did" : <issuer_did>
 }
 @endcode
 
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param proverDid DID of the prover
 @param claimOfferJSON Claim offer as a json containing information about the issuer and a claim.
 @param claimDefJSON Claim definition json associated with issuer_did and schema_seq_no in the claim_offer.
 @param masterSecretName Name of the master secret stored in the wallet
 @handler Callback that takes command result as parameter. Returns Claim request json.

 @return Error Code.
 */
+ (NSError *)proverCreateAndStoreClaimReqWithWalletHandle:(IndyHandle)walletHandle
                                                proverDid:(NSString *)proverDid
                                           claimOfferJSON:(NSString *)claimOfferJSON
                                             claimDefJSON:(NSString *)claimDefJSON
                                         masterSecretName:(NSString *)masterSecretName
                                               completion:(void (^)(NSError *error, NSString *claimReqJSON)) handler;

/**
 Updates the claim by a master secret and stores in a secure wallet.  
 
 The claim contains the information about schema_seq_no, issuer_did, revoc_reg_seq_no (see issuer_create_claim).  
 
 Seq_no is a sequence number of the corresponding transaction in the ledger.  
 
 The method loads a blinded secret for this key from the wallet, updates the claim and stores it in a wallet.
 
 @code
 Example claimsJson:
      {
         "claim": {attr1:[value, value_as_int]}
         "signature": <signature>,
         "schema_seq_no": string,
         "revoc_reg_seq_no", string
         "issuer_did", string
      }
 @endcode
 
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param claimsJson Claim json. See example above.
 
 @param handler Callback that takes command result as parameter.
 
 @return Error Code.
 */
+ (NSError *)proverStoreClaimWithWalletHandle:(IndyHandle)walletHandle
                                   claimsJSON:(NSString *)claimsJson
                                   completion:(void (^)(NSError *error)) handler;

/**
 Gets human readable claims according to the filter.  
 
 If filter is NULL, then all claims are returned.  
 
 Claims can be filtered by Issuer, claim_def and/or Schema.  
 
 @code
 Example filterJSON:
 {
    "issuer_did": string,
    "schema_seq_no": string
 }
 @endcode
 
 @code
 Example claims json returned in handler:
      [{
          "claim_uuid": <string>,
          "attrs": [{"attr_name" : "attr_value"}],
          "schema_seq_no": string,
          "issuer_did": string,
          "revoc_reg_seq_no": string,
      }]
 @endcode
 
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param filterJSON Filter for claims

 @param handler Callback that takes command result as parameter. Returns claims json. See example above.

 @return Error Code
 */
+ (NSError *)proverGetClaimsWithWalletHandle:(IndyHandle)walletHandle
                                  filterJSON:(NSString *)filterJSON
                                  completion:(void (^)(NSError *error, NSString *claimsJSON)) handler;

/**
 Gets human readable claims matching the given proof request.
 
 @code
 Example proofReqJSON:
      {
          "name": string,
          "version": string,
          "nonce": string,
          "requested_attr1_uuid": <attr_info>,
          "requested_attr2_uuid": <attr_info>,
         "requested_attr3_uuid": <attr_info>,
          "requested_predicate_1_uuid": <predicate_info>,
         "requested_predicate_2_uuid": <predicate_info>,
      }
 @endcode
 
 @code
 Example claimsJSON returned in handler:
      {
          "requested_attr1_uuid": [claim1, claim2],
          "requested_attr2_uuid": [],
          "requested_attr3_uuid": [claim3],
          "requested_predicate_1_uuid": [claim1, claim3],
          "requested_predicate_2_uuid": [claim2],
      }, where claim is
      {
          "claim_uuid": <string>,
          "attrs": [{"attr_name" : "attr_value"}],
          "schema_seq_no": string,
          "issuer_did": string,
          "revoc_reg_seq_no": string,
      }
 
 @endcode
 
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithNam).
 @param proofReqJSON Proof request json. See example above.
 @param handler Callback that takes command result as parameter. Returns json with claims for the given pool request. Claim consists of uuid, human-readable attributes (key-value map), schema_seq_no, issuer_did and revoc_reg_seq_no. See example above.
 
 @return Error Code
 */
+ (NSError *)proverGetClaimsForProofReqWithWalletHandle:(IndyHandle)walletHandle
                                           proofReqJSON:(NSString *)proofReqJSON
                                             completion:(void (^)(NSError *error, NSString *claimsJSON)) handler;

/**
 Creates a proof according to the given proof request.  
 
 Either a corresponding claim with optionally revealed attributes or self-attested attribute must be provided 
 for each requested attribute (see IndyAnoncreds::proverGetClaimsForProofReqWithWalletHandle).
 
 A proof request may request multiple claims from different schemas and different issuers.
 All required schemas, public keys and revocation registries must be provided.  
 
 The proof request also contains nonce.
 The proof contains either proof or self-attested attribute value for each requested attribute.
 
 @code
 Example proofReqJSON:
 {
        "nonce": string,
        "requested_attr1_uuid": <attr_info>,
        "requested_attr2_uuid": <attr_info>,
        "requested_attr3_uuid": <attr_info>,
        "requested_predicate_1_uuid": <predicate_info>,
        "requested_predicate_2_uuid": <predicate_info>
 }
 @endcode
 
 @code
 Example requestedClaimsJSON:
      {
          "requested_attr1_uuid": [claim1_uuid_in_wallet, true <reveal_attr>],
          "requested_attr2_uuid": [self_attested_attribute],
          "requested_attr3_uuid": [claim2_seq_no_in_wallet, false]
          "requested_attr4_uuid": [claim2_seq_no_in_wallet, true]
          "requested_predicate_1_uuid": [claim2_seq_no_in_wallet],
          "requested_predicate_2_uuid": [claim3_seq_no_in_wallet],
      }
 @endcode
 
 @code
 Example schemasJSON:
      {
         "claim1_uuid_in_wallet": <schema1>,
         "claim2_uuid_in_wallet": <schema2>,
         "claim3_uuid_in_wallet": <schema3>,
      }
 @endcode
 
 @code
 Example claimDefsJSON:
     {
        "claim1_uuid_in_wallet": <claim_def1>,
        "claim2_uuid_in_wallet": <claim_def2>,
        "claim3_uuid_in_wallet": <claim_def3>,
     }
 @endcode
 
 @code
 Example revocRegsJSON:
    {
        "claim1_uuid_in_wallet": <revoc_reg1>,
        "claim2_uuid_in_wallet": <revoc_reg2>,
        "claim3_uuid_in_wallet": <revoc_reg3>,
    }
 @endcode
 
 @code
 Example proofJSON returned in handler:
      {
          "requested": {
              "requested_attr1_id": [claim_proof1_uuid, revealed_attr1, revealed_attr1_as_int],
              "requested_attr2_id": [self_attested_attribute],
              "requested_attr3_id": [claim_proof2_uuid]
              "requested_attr4_id": [claim_proof2_uuid, revealed_attr4, revealed_attr4_as_int],
              "requested_predicate_1_uuid": [claim_proof2_uuid],
              "requested_predicate_2_uuid": [claim_proof3_uuid],
          }
          "claim_proofs": {
              "claim_proof1_uuid": [<claim_proof>, issuer_did, schema_seq_no, revoc_reg_seq_no],
              "claim_proof2_uuid": [<claim_proof>, issuer_did, schema_seq_no, revoc_reg_seq_no],
              "claim_proof3_uuid": [<claim_proof>, issuer_did, schema_seq_no, revoc_reg_seq_no]
          },
         "aggregated_proof": <aggregated_proof>
      }
 
 @endcode
 
 
 
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithNam).
 @param proofReqJSON Proof request json as come from the verifier. See example above.

 @param requestedClaimsJSON Either a claim or self-attested attribute for each requested attribute. See example above.

 @param schemasJSON All schema jsons participating in the proof request. See example above.

 
 @param masterSecretName The name of the master secret stored in the wallet.
 @param claimDefsJSON All claim definition jsons participating in the proof request. See example above.

 @param revocRegsJSON All revocation registry jsons participating in the proof request.
 
 @param handler Callback that takes command result as parameter. Returns proof json: For each requested attribute either a proof (with optionally revealed attribute value) or self-attested attribute value is provided. Each proof is associated with a claim and corresponding schema_seq_no, issuer_did and revoc_reg_seq_no. There ais also aggregated proof part common for all claim proofs.
 
 @return ErrorCode
 
 */
+ (NSError *)proverCreateProofWithWalletHandle:(IndyHandle)walletHandle
                                  proofReqJSON:(NSString *)proofReqJSON
                           requestedClaimsJSON:(NSString *)requestedClaimsJSON
                                   schemasJSON:(NSString *)schemasJSON
                              masterSecretName:(NSString *)masterSecretName
                                 claimDefsJSON:(NSString *)claimDefsJSON
                                 revocRegsJSON:(NSString *)revocRegsJSON
                                    completion:(void (^)(NSError *error, NSString *proofJSON)) handler;

/**
 Verifies a proof (of multiple claim).
 All required schemas, public keys and revocation registries must be provided.
 
 @code
 Example proofReqJSON:
  {
    "nonce": string,
    "requested_attr1_uuid": <attr_info>,
    "requested_attr2_uuid": <attr_info>,
    "requested_attr3_uuid": <attr_info>,
    "requested_predicate_1_uuid": <predicate_info>,
    "requested_predicate_2_uuid": <predicate_info>,
  }
 @endcode
 
 @code
 Example proofJSON:
      {
          "requested": {
              "requested_attr1_id": [claim_proof1_uuid, revealed_attr1, revealed_attr1_as_int],
              "requested_attr2_id": [self_attested_attribute],
              "requested_attr3_id": [claim_proof2_uuid]
              "requested_attr4_id": [claim_proof2_uuid, revealed_attr4, revealed_attr4_as_int],
              "requested_predicate_1_uuid": [claim_proof2_uuid],
              "requested_predicate_2_uuid": [claim_proof3_uuid],
          }
          "claim_proofs": {
              "claim_proof1_uuid": [<claim_proof>, issuer_did, schema_seq_no, revoc_reg_seq_no],
              "claim_proof2_uuid": [<claim_proof>, issuer_did, schema_seq_no, revoc_reg_seq_no],
              "claim_proof3_uuid": [<claim_proof>, issuer_did, schema_seq_no, revoc_reg_seq_no]
          },
          "aggregated_proof": <aggregated_proof>
      }
 @endcode
 
 @code
 Example schemasJSON:
          {
              "claim_proof1_uuid": <schema>,
              "claim_proof2_uuid": <schema>,
              "claim_proof3_uuid": <schema>
          }
 @endcode
 
 @code
 Example claimDefsJSON:
        {
              "claim_proof1_uuid": <claim_def>,
              "claim_proof2_uuid": <claim_def>,
              "claim_proof3_uuid": <claim_def>
        }
 @endcode
 
 @code
 Example revocRegsJSON:
        {
            "claim_proof1_uuid": <revoc_reg>,
            "claim_proof2_uuid": <revoc_reg>,
            "claim_proof3_uuid": <revoc_reg>
        }
 @endcode
 
 @param proofReqJSON Initial proof request as sent by the verifier. See example above.

 @param proofJSON Proof json. For each requested attribute either a proof (with optionally revealed attribute value) or
        self-attested attribute value is provided.  
 
        Each proof is associated with a claim and corresponding schema_seq_no, issuer_did and revoc_reg_seq_no.
        There ais also aggregated proof part common for all claim proofs. See example above.

 @param schemasJSON All schema jsons participating in the proof. See example above.

 @param claimDefsJSON All claim definition jsons participating in the proof. See example above.

 @param revocRegsJSON All revocation registry jsons participating in the proof.

 @param handler Callback that takes command result as parameter. Returns result flag: valid: true - if signature is valid, false - otherwise.

 @return Error Code
 */
+ (NSError *)verifierVerifyProofWithWalletHandle:(NSString *)proofReqJSON
                                       proofJSON:(NSString *)proofJSON
                                     schemasJSON:(NSString *)schemasJSON
                                   claimDefsJSON:(NSString *)claimDefsJSON
                                   revocRegsJSON:(NSString *)revocRegsJSON
                                      completion:(void (^)(NSError *error, BOOL valid)) handler;
@end
