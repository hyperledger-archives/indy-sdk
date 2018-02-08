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
 
 @param issuerDID DID of the issuer signing claim_def transaction to the Ledger
 @param schemaJSON Schema as a json
 @param signatureType Signature type (optional). Currently only 'CL' is supported.
 @param createNonRevoc Whether to request non-revocation claim.
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter. Returns claim definition json containing information about signature type, schema and issuer's public key. Unique number identifying the public key in the wallet.
*/
+ (void)issuerCreateAndStoreClaimDefForIssuerDID:(NSString *)issuerDID
                                      schemaJSON:(NSString *)schemaJSON
                                   signatureType:(NSString *)signatureType
                                  createNonRevoc:(BOOL)createNonRevoc
                                    walletHandle:(IndyHandle)walletHandle
                                      completion:(void (^)(NSError *error, NSString *claimDefJSON))completion;

/**
 Creates a new revocation registry for the given claim definition.
 Stores it in a secure wallet identifying by the returned key.
 
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param issuerDID DID of the issuer signing revoc_reg transaction to the Ledger
 @param schemaSeqNo Seq no of a schema transaction in Ledger
 @param maxClaimNum Maximum number of claims the new registry can process.
 @param completion Callback that takes command result as parameter. Returns revoc registry json and unique number identifying the revocation registry in the wallet.
 */
+ (void)issuerCreateAndStoreRevocRegForIssuerDid:(NSString *)issuerDID
                                      schemaJSON:(NSString *)schemaJSON
                                     maxClaimNum:(NSNumber *)maxClaimNum
                                    walletHandle:(IndyHandle)walletHandle
                                      completion:(void (^)(NSError *error, NSString *revocRegJSON))completion;

/**
 Create claim offer and store it in wallet.

 @param issuerDID DID of the issuer signing claim_def transaction to the Ledger
 @param issuerDID DID of the targer user
 @param schemaJSON Schema as a json
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter.
 Returns claim offer json
     claim offer json:
       {
            "issuer_did": string,
            "schema_key" : {name: string, version: string, did: string},
            "nonce": string,
            "key_correctness_proof" : <key_correctness_proof>
       }
*/
+ (void)issuerCreateClaimOfferForProverDID:(NSString *)proverDID
                                 issuerDID:(NSString *)issuerDID
                                schemaJSON:(NSString *)schemaJSON
                              walletHandle:(IndyHandle)walletHandle
                                completion:(void (^)(NSError *error, NSString *claimOfferJSON))completion;

/**
 Signs a given claim for the given user by a given key (claim ef).
 The corresponding claim definition and revocation registry must be already created
 an stored into the wallet.

 @code
 Example claimReqJSON:
 {
 "blinded_ms" : <blinded_master_secret>,
 "schema_seq_no" : <schema_seq_no>,
 "issuer_DID" : <issuer_DID>
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
 "issuer_DID", string,
 "schema_seq_no", string,
 }
 
 @endcode

 @param claimRequestJSON Claim request with a blinded secret from the user (returned by IndyAnoncreds::proverCreateAndStoreClaimReqWithClaimDef).
        Also contains schema_seq_no and issuer_DID.
 
 @param claimJSON Claim containing attribute values for each of requested attribute names.
 
 @param userRevocIndex Index of a new user in the revocation registry (optional, pass -1 if user_revoc_index is absentee; default one is used if not provided)
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter. Returns revocation registry update json with a newly issued claim and claim json containing issued claim, issuer_DID, schema_seq_no, and revoc_reg_seq_no
 used for issuance.
 */
+ (void)issuerCreateClaimWithRequest:(NSString *)claimRequestJSON
                           claimJSON:(NSString *)claimJSON
                      userRevocIndex:(NSNumber *)userRevocIndex
                        walletHandle:(IndyHandle)walletHandle
                          completion:(void (^)(NSError *error, NSString *revocRegUpdateJSON, NSString *xclaimJSON))completion;

/**
 Revokes a user identified by a revoc_id in a given revoc-registry.
 The corresponding claim definition and revocation registry must be already
 created an stored into the wallet.
 
 @param issuerDID DID of the issuer signing claim_def transaction to the Ledger.
 @param schemaSeqNo Seq no of a schema transaction in Ledger.
 @param userRevocIndex Index of the user in the revocation registry.
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter. Returns revocation registry update json with a revoked claim.
 */
+ (void)issuerRevokeClaimForIssuerDID:(NSString *)issuerDID
                           schemaJSON:(NSString *)schemaJSON
                       userRevocIndex:(NSNumber *)userRevocIndex
                         walletHandle:(IndyHandle)walletHandle
                           completion:(void (^)(NSError *error, NSString *revocRegUpdateJSON))completion;

/**
 Stores a claim offer from the given issuer in a secure storage.
 
 @code
 Example claimOfferJSON:
 {
    "issuer_DID": string,
    "schema_seq_no": string
 }
 @endcode
 
 @param claimOfferJSON Claim offer as a json containing information about the issuer and a claim.
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 */
+ (void)proverStoreClaimOffer:(NSString *)claimOfferJSON
             WithWalletHandle:(IndyHandle)walletHandle
                   completion:(void (^)(NSError *error))completion;

/**
 Gets all stored claim offers (see IndyAnoncreds::proverStoreClaimOfferWithWalletHandle).
 A filter can be specified to get claim offers for specific Issuer, claim_def or schema only.
 
 @code
 Example filterJSON:
 {
 "issuer_DID": string,
 "schema_seq_no": string
 }
 @endcode
 
 @code
 Example claimOffersJSON:
 {
 [{"issuer_DID": string,
 "schema_seq_no": string}]
 }
 @endcode

 @param filterJSON Optional filter to get claim offers for specific Issuer, claim_def or schema only only
 Each of the filters is optional.
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Returns A json with a list of claim offers for the filter.
 */
+ (void)proverGetClaimOffersWithFilter:(NSString *)filterJSON
                          walletHandle:(IndyHandle)walletHandle
                            completion:(void (^)(NSError *error, NSString *claimOffersJSON))completion;


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
 "issuer_DID": string,
 "schema_seq_no": string
 }
 @endcode
 
 @code
 Example claimReqJSON returned in handle:
 {
 "blinded_ms" : <blinded_master_secret>,
 "schema_seq_no" : <schema_seq_no>,
 "issuer_DID" : <issuer_DID>
 }
 @endcode
 
 @param claimDefJSON Claim definition json associated with issuer_DID and schema_seq_no in the claim_offer.
 @param proverDID DID of the prover
 @param claimOfferJSON Claim offer as a json containing information about the issuer and a claim.
 @param masterSecretName Name of the master secret stored in the wallet
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter. Returns Claim request json.
 */
+ (void)proverCreateAndStoreClaimReqWithClaimDef:(NSString *)claimDefJSON
                                       proverDID:(NSString *)proverDID
                                  claimOfferJSON:(NSString *)claimOfferJSON
                                masterSecretName:(NSString *)masterSecretName
                                    walletHandle:(IndyHandle)walletHandle
                                      completion:(void (^)(NSError *error, NSString *claimReqJSON))completion;

/**
 Updates the claim by a master secret and stores in a secure wallet.  
 
 The claim contains the information about schema_seq_no, issuer_DID, revoc_reg_seq_no (see issuer_create_claim).  
 
 Seq_no is a sequence number of the corresponding transaction in the ledger.  
 
 The method loads a blinded secret for this key from the wallet, updates the claim and stores it in a wallet.
 
 @code
 Example claimsJson:
      {
         "claim": {attr1:[value, value_as_int]}
         "signature": <signature>,
         "schema_seq_no": string,
         "revoc_reg_seq_no", string
         "issuer_DID", string
      }
 @endcode
 
 @param claimsJson Claim json. See example above.
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param revRegJSON Revocation registry json associated with issuer_DID and schema_seq_no in the claim_offer.
 @param completion Callback that takes command result as parameter.
 */
+ (void)proverStoreClaim:(NSString *)claimsJson
              revRegJSON:(NSString *)revRegJSON
            walletHandle:(IndyHandle)walletHandle
              completion:(void (^)(NSError *error))completion;

/**
 Gets human readable claims according to the filter.  
 
 If filter is NULL, then all claims are returned.  
 
 Claims can be filtered by Issuer, claim_def and/or Schema.  
 
 @code
 Example filterJSON:
 {
    "issuer_DID": string,
    "schema_seq_no": string
 }
 @endcode
 
 @code
 Example claims json returned in handler:
      [{
          "referent": <string>,
          "attrs": [{"attr_name" : "attr_value"}],
          "schema_seq_no": string,
          "issuer_DID": string,
          "revoc_reg_seq_no": string,
      }]
 @endcode
 
 @param filterJSON Filter for claims
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter. Returns claims json. See example above.

 */
+ (void)proverGetClaimsWithFilter:(NSString *)filterJSON
                     walletHandle:(IndyHandle)walletHandle
                       completion:(void (^)(NSError *error, NSString *claimsJSON))completion;

/**
 Gets human readable claims matching the given proof request.
 
 @code
 Example proofReqJSON:
      {
          "name": string,
          "version": string,
          "nonce": string,
          "requested_attr1_referent": <attr_info>,
          "requested_attr2_referent": <attr_info>,
         "requested_attr3_referent": <attr_info>,
          "requested_predicate_1_referent": <predicate_info>,
         "requested_predicate_2_referent": <predicate_info>,
      }
 @endcode
 
 @code
 Example claimsJSON returned in handler:
      {
          "requested_attr1_referent": [claim1, claim2],
          "requested_attr2_referent": [],
          "requested_attr3_referent": [claim3],
          "requested_predicate_1_referent": [claim1, claim3],
          "requested_predicate_2_referent": [claim2],
      }, where claim is
      {
          "referent": <string>,
          "attrs": [{"attr_name" : "attr_value"}],
          "schema_seq_no": string,
          "issuer_DID": string,
          "revoc_reg_seq_no": string,
      }
 
 @endcode
 
 @param proofReqJSON Proof request json. See example above.
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithNam).
 @param completion Callback that takes command result as parameter. Returns json with claims for the given pool request. Claim consists of uuid, human-readable attributes (key-value map), schema_seq_no, issuer_DID and revoc_reg_seq_no. See example above.
 
 */
+ (void)proverGetClaimsForProofReq:(NSString *)proofReqJSON
                      walletHandle:(IndyHandle)walletHandle
                        completion:(void (^)(NSError *error, NSString *claimsJSON))completion;

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
        "requested_attr1_referent": <attr_info>,
        "requested_attr2_referent": <attr_info>,
        "requested_attr3_referent": <attr_info>,
        "requested_predicate_1_referent": <predicate_info>,
        "requested_predicate_2_referent": <predicate_info>
 }
 @endcode
 
 @code
 Example requestedClaimsJSON:
      {
          "requested_attr1_referent": [claim1_referent_in_wallet, true <reveal_attr>],
          "requested_attr2_referent": [self_attested_attribute],
          "requested_attr3_referent": [claim2_seq_no_in_wallet, false]
          "requested_attr4_referent": [claim2_seq_no_in_wallet, true]
          "requested_predicate_1_referent": [claim2_seq_no_in_wallet],
          "requested_predicate_2_referent": [claim3_seq_no_in_wallet],
      }
 @endcode
 
 @code
 Example schemasJSON:
      {
         "claim1_referent_in_wallet": <schema1>,
         "claim2_referent_in_wallet": <schema2>,
         "claim3_referent_in_wallet": <schema3>,
      }
 @endcode
 
 @code
 Example claimDefsJSON:
     {
        "claim1_referent_in_wallet": <claim_def1>,
        "claim2_referent_in_wallet": <claim_def2>,
        "claim3_referent_in_wallet": <claim_def3>,
     }
 @endcode
 
 @code
 Example revocRegsJSON:
    {
        "claim1_referent_in_wallet": <revoc_reg1>,
        "claim2_referent_in_wallet": <revoc_reg2>,
        "claim3_referent_in_wallet": <revoc_reg3>,
    }
 @endcode
 
 @code
 Example proofJSON returned in handler:
      {
          "requested": {
              "requested_attr1_id": [claim_proof1_referent, revealed_attr1, revealed_attr1_as_int],
              "requested_attr2_id": [self_attested_attribute],
              "requested_attr3_id": [claim_proof2_referent]
              "requested_attr4_id": [claim_proof2_referent, revealed_attr4, revealed_attr4_as_int],
              "requested_predicate_1_referent": [claim_proof2_referent],
              "requested_predicate_2_referent": [claim_proof3_referent],
          }
          "claim_proofs": {
              "claim_proof1_referent": [<claim_proof>, issuer_DID, schema_seq_no, revoc_reg_seq_no],
              "claim_proof2_referent": [<claim_proof>, issuer_DID, schema_seq_no, revoc_reg_seq_no],
              "claim_proof3_referent": [<claim_proof>, issuer_DID, schema_seq_no, revoc_reg_seq_no]
          },
         "aggregated_proof": <aggregated_proof>
      }
 
 @endcode
 
 @param proofRequestJSON Proof request json as come from the verifier. See example above.

 @param requestedClaimsJSON Either a claim or self-attested attribute for each requested attribute. See example above.

 @param schemasJSON All schema jsons participating in the proof request. See example above.

 
 @param masterSecretName The name of the master secret stored in the wallet.
 @param claimDefsJSON All claim definition jsons participating in the proof request. See example above.

 @param revocRegsJSON All revocation registry jsons participating in the proof request.
 
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithNam).
 
 @param completion Callback that takes command result as parameter. Returns proof json: For each requested attribute either a proof (with optionally revealed attribute value) or self-attested attribute value is provided. Each proof is associated with a claim and corresponding schema_seq_no, issuer_DID and revoc_reg_seq_no. There ais also aggregated proof part common for all claim proofs.
 */
+ (void)proverCreateProofForRequest:(NSString *)proofRequestJSON
                requestedClaimsJSON:(NSString *)requestedClaimsJSON
                        schemasJSON:(NSString *)schemasJSON
                   masterSecretName:(NSString *)masterSecretName
                      claimDefsJSON:(NSString *)claimDefsJSON
                      revocRegsJSON:(NSString *)revocRegsJSON
                       walletHandle:(IndyHandle)walletHandle
                         completion:(void (^)(NSError *error, NSString *proofJSON))completion;

/**
 Verifies a proof (of multiple claim).
 All required schemas, public keys and revocation registries must be provided.
 
 @code
 Example proofReqJSON:
  {
    "nonce": string,
    "requested_attr1_referent": <attr_info>,
    "requested_attr2_referent": <attr_info>,
    "requested_attr3_referent": <attr_info>,
    "requested_predicate_1_referent": <predicate_info>,
    "requested_predicate_2_referent": <predicate_info>,
  }
 @endcode
 
 @code
 Example proofJSON:
      {
          "requested": {
              "requested_attr1_id": [claim_proof1_referent, revealed_attr1, revealed_attr1_as_int],
              "requested_attr2_id": [self_attested_attribute],
              "requested_attr3_id": [claim_proof2_referent]
              "requested_attr4_id": [claim_proof2_referent, revealed_attr4, revealed_attr4_as_int],
              "requested_predicate_1_referent": [claim_proof2_referent],
              "requested_predicate_2_referent": [claim_proof3_referent],
          }
          "claim_proofs": {
              "claim_proof1_referent": [<claim_proof>, issuer_DID, schema_seq_no, revoc_reg_seq_no],
              "claim_proof2_referent": [<claim_proof>, issuer_DID, schema_seq_no, revoc_reg_seq_no],
              "claim_proof3_referent": [<claim_proof>, issuer_DID, schema_seq_no, revoc_reg_seq_no]
          },
          "aggregated_proof": <aggregated_proof>
      }
 @endcode
 
 @code
 Example schemasJSON:
          {
              "claim_proof1_referent": <schema>,
              "claim_proof2_referent": <schema>,
              "claim_proof3_referent": <schema>
          }
 @endcode
 
 @code
 Example claimDefsJSON:
        {
              "claim_proof1_referent": <claim_def>,
              "claim_proof2_referent": <claim_def>,
              "claim_proof3_referent": <claim_def>
        }
 @endcode
 
 @code
 Example revocRegsJSON:
        {
            "claim_proof1_referent": <revoc_reg>,
            "claim_proof2_referent": <revoc_reg>,
            "claim_proof3_referent": <revoc_reg>
        }
 @endcode
 
 @param proofRequestJson Initial proof request as sent by the verifier. See example above.

 @param proofJSON Proof json. For each requested attribute either a proof (with optionally revealed attribute value) or
        self-attested attribute value is provided.  
 
        Each proof is associated with a claim and corresponding schema_seq_no, issuer_DID and revoc_reg_seq_no.
        There ais also aggregated proof part common for all claim proofs. See example above.

 @param schemasJSON All schema jsons participating in the proof. See example above.

 @param claimDefsJSON All claim definition jsons participating in the proof. See example above.

 @param revocRegsJSON All revocation registry jsons participating in the proof.

 @param completion Callback that takes command result as parameter. Returns result flag: valid: true - if signature is valid, false - otherwise.

 */
+ (void)verifierVerifyProofRequest:(NSString *)proofRequestJson
                         proofJSON:(NSString *)proofJSON
                       schemasJSON:(NSString *)schemasJSON
                     claimDefsJSON:(NSString *)claimDefsJSON
                     revocRegsJSON:(NSString *)revocRegsJSON
                        completion:(void (^)(NSError *error, BOOL valid))completion;
@end
