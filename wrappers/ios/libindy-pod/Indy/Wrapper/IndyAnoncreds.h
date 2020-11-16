//
//  IndyAnoncreds.h
//  libindy
//


#import <Foundation/Foundation.h>
#import "IndyTypes.h"

@interface IndyAnoncreds : NSObject

/**
These functions wrap the Ursa algorithm as documented in this paper:
https://github.com/hyperledger/ursa/blob/master/libursa/docs/AnonCred.pdf

And is documented in this HIPE:
https://github.com/hyperledger/indy-hipe/blob/c761c583b1e01c1e9d3ceda2b03b35336fdc8cc1/text/anoncreds-protocol/README.md
*.

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
 @param attrs a list of schema attributes descriptions (the number of attributes should be less or equal than 125)
 @param completion Callback that takes command result as parameter.
 Returns:
    schemaId: identifier of created schema
    schemaJson: schema as json
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
    - 'CL': Camenisch-Lysyanskaya credential signature type that is implemented according to the algorithm in this paper:
                https://github.com/hyperledger/ursa/blob/master/libursa/docs/AnonCred.pdf
            And is documented in this HIPE:
                https://github.com/hyperledger/indy-hipe/blob/c761c583b1e01c1e9d3ceda2b03b35336fdc8cc1/text/anoncreds-protocol/README.md

 @param configJSON: type-specific configuration of credential definition as json:
 - 'CL':
   - revocationSupport: whether to request non-revocation credential (optional, default false)
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter.

 Note: Use combination of `issuerRotateCredentialDefStartForId` and `issuerRotateCredentialDefApplyForId` functions
 to generate new keys for an existing credential definition

 Returns:
    credDefId: identifier of created credential definition.
    credDefJson: public part of created credential definition
   {
       id: string - identifier of credential definition
       schemaId: string - identifier of stored in ledger schema
       type: string - type of the credential definition. CL is the only supported type now.
       tag: string - allows to distinct between credential definitions for the same issuer and schema
       value: Dictionary with Credential Definition's data is depended on the signature type: {
           primary: primary credential public key,
           Optional<revocation>: revocation credential public key
       },
       ver: Version of the CredDef json
   }
   
   Note: `primary` and `revocation` fields of credential definition are complex opaque types that contain data structures internal to Ursa.
   They should not be parsed and are likely to change in future versions.
*/
+ (void)issuerCreateAndStoreCredentialDefForSchema:(NSString *)schemaJSON
                                         issuerDID:(NSString *)issuerDID
                                               tag:(NSString *)tag
                                              type:(NSString *)type
                                        configJSON:(NSString *)configJSON
                                      walletHandle:(IndyHandle)walletHandle
                                        completion:(void (^)(NSError *error, NSString *credDefId, NSString *credDefJSON))completion;

/**
 Generate temporary credential definitional keys for an existing one (owned by the caller of the library).

 Use `issuerRotateCredentialDefApplyForId` function to set temporary keys as the main.

 WARNING: Rotating the credential definitional keys will result in making all credentials issued under the previous keys unverifiable.

 @param credDefId an identifier of created credential definition stored in the wallet
 @param configJSON: type-specific configuration of credential definition as json:
 - 'CL':
   - revocationSupport: whether to request non-revocation credential (optional, default false)
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter.
 Returns:
    credDefJson: public part of temporary created credential definition
*/
+ (void)issuerRotateCredentialDefStartForId:(NSString *)credDefId
                                 configJSON:(NSString *)configJSON
                               walletHandle:(IndyHandle)walletHandle
                                 completion:(void (^)(NSError *error, NSString *credDefJSON))completion;

/**
 Apply temporary keys as main for an existing Credential Definition (owned by the caller of the library).

 WARNING: Rotating the credential definitional keys will result in making all credentials issued under the previous keys unverifiable.

 @param credDefId an identifier of created credential definition stored in the wallet
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter.
 Returns: void
*/
+ (void)issuerRotateCredentialDefApplyForId:(NSString *)credDefId
                               walletHandle:(IndyHandle)walletHandle
                                 completion:(void (^)(NSError *error))completion;

/**
 Create a new revocation registry for the given credential definition as tuple of entities:
 - Revocation registry definition that encapsulates credentials definition reference, revocation type specific configuration and
   secrets used for credentials revocation
 - Revocation registry state that stores the information about revoked entities in a non-disclosing way. The state can be
   represented as ordered list of revocation registry entries were each entry represents the list of revocation or issuance operations.

 Revocation registry definition entity contains private and public parts. Private part will be stored in the wallet. Public part
 will be returned as json intended to be shared with all anoncreds workflow actors usually by publishing REVOC_REG_DEF transaction
 to Indy distributed ledger.

 Revocation registry state is stored on the wallet and also intended to be shared as the ordered list of REVOC_REG_ENTRY transactions.
 This call initializes the state in the wallet and returns the initial entry.

 Some revocation registry types (for example, 'CL_ACCUM') can require generation of binary blob called tails used to hide information about revoked credentials in public
 revocation registry and intended to be distributed out of leger (REVOC_REG_DEF transaction will still contain uri and hash of tails).
 This call requires access to pre-configured blob storage writer instance handle that will allow to write generated tails.
 
 @param walletHandle: wallet handler (created by open_wallet).
 @param issuerDID: a DID of the issuer signing transaction to the Ledger
 @param type: (optional, default value depends on credential definition type). Supported types are:
               - 'CL_ACCUM': Type-3 pairing based accumulator implemented according to the algorithm in this paper:
                                 https://github.com/hyperledger/ursa/blob/master/libursa/docs/AnonCred.pdf
                             This type is default for 'CL' credential definition type.
 @param tag: allows to distinct between revocation registries for the same issuer and credential definition
 @param credDefID: id of stored in ledger credential definition
 @param configJSON: type-specific configuration of revocation registry as json:
     - 'CL_ACCUM': {
         "issuance_type": (optional) type of issuance. Currently supported:
             1) ISSUANCE_BY_DEFAULT: all indices are assumed to be issued and initial accumulator is calculated over all indices;
                Revocation Registry is updated only during revocation.
             2) ISSUANCE_ON_DEMAND: nothing is issued initially accumulator is 1 (used by default);
         "max_cred_num": maximum number of credentials the new registry can process (optional, default 100000)
     }
 @param tailsWriterHandle: handle of blob storage to store tails
 @param completion Callback that takes command result as parameter.
 
 NOTE:
     Recursive creation of folder for Default Tails Writer (correspondent to `tailsWriterHandle`)
     in the system-wide temporary directory may fail in some setup due to permissions: `IO error: Permission denied`.
     In this case use `TMPDIR` environment variable to define temporary directory specific for an application.
 
 Returns 
    revocRegID: identifier of created revocation registry definition
    revocRegDefJSON: public part of revocation registry definition
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
               "publicKeys": <public_keys> - Registry's public key (opaque type that contains data structures internal to Ursa.
                                                                    It should not be parsed and are likely to change in future versions).
           },
           "ver": string - version of revocation registry definition json.
       }
    revocRegEntryJSON: revocation registry entry that defines initial state of revocation registry
       {
           value: {
               prevAccum: string - previous accumulator value.
               accum: string - current accumulator value.
               issued: array<number> - an array of issued indices.
               revoked: array<number> an array of revoked indices.
           },
           ver: string - version revocation registry entry json
       }
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
 Create credential offer that will be used by Prover for
 credential request creation. Offer includes nonce and key correctness proof
 for authentication between protocol steps and integrity checking.

 @param credDefID: id of credential definition stored in the wallet
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter.
 Returns credential offer json:
     {
         "schema_id": string,
         "cred_def_id": string,
         // Fields below can depend on Cred Def type
         "nonce": string,
         "key_correctness_proof" : key correctness proof for credential definition correspondent to cred_def_id
                                   (opaque type that contains data structures internal to Ursa.
                                   It should not be parsed and are likely to change in future versions).
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
     If you want to use empty value for some credential field, you should set "raw" to "" and "encoded" should not be empty
 @param revRegId: (Optional) id of stored revocation registry definition
 @param blobStorageReaderHandle: (Optional) Pre-configured blob storage reader instance handle that will allow to read revocation tails
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter. 
 Returns:
     credJSON: Credential json containing signed credential values
        {
            "schema_id": string,
            "cred_def_id": string,
            "rev_reg_def_id", Optional<string>,
            "values": <see cred_values_json above>,
            // Fields below can depend on Cred Def type
            "signature": <credential signature>,
                         (opaque type that contains data structures internal to Ursa.
                          It should not be parsed and are likely to change in future versions).
            "signature_correctness_proof": credential signature correctness proof
                         (opaque type that contains data structures internal to Ursa.
                          It should not be parsed and are likely to change in future versions).
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
 @param completion Callback that takes command result as parameter. 
 Returns id of generated master secret.
 
 */
+ (void)proverCreateMasterSecret:(NSString *)masterSecretID
                    walletHandle:(IndyHandle)walletHandle
                      completion:(void (^)(NSError *error, NSString *outMasterSecretId))completion;

/**
 Creates a credential request for the given credential offer.

 The method creates a blinded master secret for a master secret identified by a provided name.
 The master secret identified by the name must be already stored in the secure wallet (see proverCreateMasterSecret)
 The blinded master secret is a part of the credential request.

 @param proverDID: a DID of the prover
 @param credOfferJSON: credential offer as a json containing information about the issuer and a credential
 @param credentialDefJSON: credential definition json related to <cred_def_id> in <credOfferJSON> 
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
                    (opaque type that contains data structures internal to Ursa.
                     It should not be parsed and are likely to change in future versions).
      "blinded_ms_correctness_proof" : <blinded_ms_correctness_proof>,
                    (opaque type that contains data structures internal to Ursa.
                     It should not be parsed and are likely to change in future versions).
      "nonce": string
    }
    credReqMetadataJSON: Credential request metadata json for further processing of received form Issuer credential.
        Note: credReqMetadataJSON mustn't be shared with Issuer.
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

 To support efficient and flexible search the following tags will be created for stored credential:
     {
         "schema_id": <credential schema id>,
         "schema_issuer_did": <credential schema issuer did>,
         "schema_name": <credential schema name>,
         "schema_version": <credential schema version>,
         "issuer_did": <credential issuer did>,
         "cred_def_id": <credential definition id>,
         "rev_reg_id": <credential revocation registry id>, // "None" as string if not present
         // for every attribute in <credential values>
         "attr::<attribute name>::marker": "1",
         "attr::<attribute name>::value": <attribute raw value>,
     }
 
 @param credID: (optional, default is a random one) identifier by which credential will be stored in the wallet
 @param credReqMetadataJSON: a credential request metadata created by proverCreateCredentialReqForCredentialOffer
 @param credJson:  credential json received from issuer
 @param credDefJSON: credential definition json related to <cred_def_id> in <credJson>
 @param revRegDefJSON: revocation registry definition json related to <rev_reg_def_id> in <credJson>
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter.
 Returns
     outCredID: identifier by which credential is stored in the wallet
 */
+ (void)proverStoreCredential:(NSString *)credJson
                       credID:(NSString *)credID
          credReqMetadataJSON:(NSString *)credReqMetadataJSON
                  credDefJSON:(NSString *)credDefJSON
                revRegDefJSON:(NSString *)revRegDefJSON
                 walletHandle:(IndyHandle)walletHandle
                   completion:(void (^)(NSError *error, NSString *outCredID))completion;

/**
 Gets human readable credential by the given id.

 @param credId: Identifier by which requested credential is stored in the wallet
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter.
 Returns credential json:
    {
        "referent": string, - id of credential in the wallet
        "attrs": {"key1":"raw_value1", "key2":"raw_value2"}, - credential attributes
        "schema_id": string, - identifier of schema
        "cred_def_id": string, - identifier of credential definition
        "rev_reg_id": Optional<string>, - identifier of revocation registry definition
        "cred_rev_id": Optional<string> - identifier of credential in the revocation registry definition
     }

 */
+ (void)proverGetCredentialWithId:(NSString *)credId
                     walletHandle:(IndyHandle)walletHandle
                       completion:(void (^)(NSError *error, NSString *credentialJSON))completion;

/**
 Deletes credential by given id.

 @param credId: Identifier by which requested credential is stored in the wallet
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param completion Callback that takes command result as parameter.
 Returns void
 */
+ (void)proverDeleteCredentialsWithId:(NSString *)credId
                         walletHandle:(IndyHandle)walletHandle
                           completion:(void (^)(NSError * error))completion;

/**
 Gets human readable credentials according to the filter.
 If filter is NULL, then all credentials are returned.
 Credentials can be filtered by tags created during saving of credential.

 NOTE: This method is deprecated because immediately returns all fetched credentials. 
 Use <proverSearchCredentialsForQuery> to fetch records by small batches.

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
        "referent": string, - id of credential in the wallet
        "attrs": {"key1":"raw_value1", "key2":"raw_value2"}, - credential attributes
        "schema_id": string, - identifier of schema
        "cred_def_id": string, - identifier of credential definition
        "rev_reg_id": Optional<string>, - identifier of revocation registry definition
        "cred_rev_id": Optional<string> - identifier of credential in the revocation registry definition
     }]

 */
+ (void)proverGetCredentialsForFilter:(NSString *)filterJSON
                         walletHandle:(IndyHandle)walletHandle
                           completion:(void (^)(NSError *error, NSString *credentialsJSON))completion;

/**
 Search for credentials stored in wallet.
 Credentials can be filtered by tags created during saving of credential.

 Instead of immediately returning of fetched credentials
 this call returns search_handle that can be used later
 to fetch records by small batches (with proverFetchCredentialsWithSearchHandle).
  
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param queryJSON Wql style filter for credentials searching based on tags.
        (indy-sdk/docs/design/011-wallet-query-language/README.md)
 @param completion Callback that takes command result as parameter. 
 Returns 
    searchHandle: Search handle that can be used later to fetch records by small batches (with proverFetchCredentialsWithSearchHandle)
    totalCount: Total count of records

 */
+ (void)proverSearchCredentialsForQuery:(NSString *)queryJSON
                            walletHandle:(IndyHandle)walletHandle
                              completion:(void (^)(NSError *error, IndyHandle searchHandle, NSNumber *totalCount))completion;

/**
 Fetch next credentials for search.
  
 @param searchHandle Search handle (created by proverSearchCredentialsForQuery).
 @param count Count of credentials to fetch
 @param completion Callback that takes command result as parameter. 
 Returns 
    credentialsJson: List of human readable credentials:
      [{
        "referent": string, - id of credential in the wallet
        "attrs": {"key1":"raw_value1", "key2":"raw_value2"}, - credential attributes
        "schema_id": string, - identifier of schema
        "cred_def_id": string, - identifier of credential definition
        "rev_reg_id": Optional<string>, - identifier of revocation registry definition
        "cred_rev_id": Optional<string> - identifier of credential in the revocation registry definition
      }]

      NOTE: The list of length less than the requested count means credentials search iterator is completed.
 */
+ (void)proverFetchCredentialsWithSearchHandle:(IndyHandle)searchHandle
                                         count:(NSNumber *)count
                                    completion:(void (^)(NSError *error, NSString *credentialsJson))completion;

/**
 Close credentials search (make search handle invalid)
  
 @param searchHandle Search handle (created by proverSearchCredentialsForQuery).
 Returns no result

 */
+ (void)proverCloseCredentialsSearchWithHandle:(IndyHandle)searchHandle
                                    completion:(void (^)(NSError *error))completion;

/**
 Gets human readable credentials matching the given proof request.

 NOTE: This method is deprecated because immediately returns all fetched credentials. 
 Use <proverSearchCredentialsForProofRequest> to fetch records by small batches.

 @param  proofReqJSON: proof request json
    {
        "name": string,
        "version": string,
        "nonce": string, - a decimal number represented as a string (use `indy_generate_nonce` function to generate 80-bit number)
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
                       // (applies to every attribute and predicate but can be overridden on attribute level)
        "ver": Optional<str>  - proof request version:
            - omit to use unqualified identifiers for restrictions
            - "1.0" to use unqualified identifiers for restrictions
            - "2.0" to use fully qualified identifiers for restrictions
    }
 attr_referent: Proof-request local identifier of requested attribute
 attr_info: Describes requested attribute
     {
         "name": Optional<string>, // attribute name, (case insensitive and ignore spaces)
         "names": Optional<[string, string]>, // attribute names, (case insensitive and ignore spaces)
                                              // NOTE: should either be "name" or "names", not both and not none of them.
                                              // Use "names" to specify several attributes that have to match a single credential.
         "restrictions": Optional<filter_json>, // see below
         "non_revoked": Optional<<non_revoc_interval>>, // see below,
                        // If specified prover must proof non-revocation
                        // for date in this interval this attribute
                        // (overrides proof level interval)
     }
 predicate_referent: Proof-request local identifier of requested attribute predicate
 predicate_info: Describes requested attribute predicate
     {
         "name": attribute name, (case insensitive and ignore spaces)
         "p_type": predicate type (">=", ">", "<=", "<")
         "p_value": int predicate value
         "restrictions": Optional<filter_json>, // see below
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
  filter_json:
     {
        "schema_id": string, (Optional)
        "schema_issuer_did": string, (Optional)
        "schema_name": string, (Optional)
        "schema_version": string, (Optional)
        "issuer_did": string, (Optional)
        "cred_def_id": string, (Optional)
     }
     
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithNam).
 @param completion Callback that takes command result as parameter. Returns json with credentials for the given proof request.
     {
         "attrs": {
             "<attr_referent>": [{ cred_info: <credential_info>, interval: Optional<non_revoc_interval> }],
             ...,
         },
         "predicates": {
             "requested_predicates": [{ cred_info: <credential_info>, timestamp: Optional<integer> }, { cred_info: <credential_2_info>, timestamp: Optional<integer> }],
             "requested_predicate_2_referent": [{ cred_info: <credential_2_info>, timestamp: Optional<integer> }]
         }
     }, where <credential_info> is
     {
         "referent": string, - id of credential in the wallet
         "attrs": {"key1":"raw_value1", "key2":"raw_value2"}, - credential attributes
         "schema_id": string, - identifier of schema
         "cred_def_id": string, - identifier of credential definition
         "rev_reg_id": Optional<string>, - identifier of revocation registry definition
         "cred_rev_id": Optional<string> - identifier of credential in the revocation registry definition
     }
 */
+ (void)proverGetCredentialsForProofReq:(NSString *)proofReqJSON
                           walletHandle:(IndyHandle)walletHandle
                             completion:(void (^)(NSError *error, NSString *credentialsJSON))completion;

/**
 Search for credentials matching the given proof request.

 Instead of immediately returning of fetched credentials
 this call returns search_handle that can be used later
 to fetch records by small batches (with proverFetchCredentialsForProofReqItemReferent).

 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithName).
 @param proofReqJSON: proof request json
    {
         "name": string,
         "version": string,
         "nonce": string, - a decimal number represented as a string (use `indy_generate_nonce` function to generate 80-bit number)
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
                        // (applies to every attribute and predicate but can be overridden on attribute level)
                        // (can be overridden on attribute level)
        "ver": Optional<str>  - proof request version:
            - omit to use unqualified identifiers for restrictions
            - "1.0" to use unqualified identifiers for restrictions
            - "2.0" to use fully qualified identifiers for restrictions
    }
 @param extraQueryJSON: (Optional) List of extra queries that will be applied to correspondent attribute/predicate:
    {
        "<attr_referent>": <wql query>,
        "<predicate_referent>": <wql query>,
    }
    
 attr_info: Describes requested attribute
     {
         "name": Optional<string>, // attribute name, (case insensitive and ignore spaces)
         "names": Optional<[string, string]>, // attribute names, (case insensitive and ignore spaces)
                                              // NOTE: should either be "name" or "names", not both and not none of them.
                                              // Use "names" to specify several attributes that have to match a single credential.
         "restrictions": Optional<filter_json>, // see below
         "non_revoked": Optional<<non_revoc_interval>>, // see below,
                        // If specified prover must proof non-revocation
                        // for date in this interval this attribute
                        // (overrides proof level interval)
     }
 predicate_referent: Proof-request local identifier of requested attribute predicate
 predicate_info: Describes requested attribute predicate
     {
         "name": attribute name, (case insensitive and ignore spaces)
         "p_type": predicate type (">=", ">", "<=", "<")
         "p_value": predicate value
         "restrictions": Optional<wql query>, // see below
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
 extra_query_json:(Optional) List of extra queries that will be applied to correspondent attribute/predicate:
     {
         "<attr_referent>": <wql query>,
         "<predicate_referent>": <wql query>,
     }
 where wql query: indy-sdk/docs/design/011-wallet-query-language/README.md
     The list of allowed fields:
         "schema_id": <credential schema id>,
         "schema_issuer_did": <credential schema issuer did>,
         "schema_name": <credential schema name>,
         "schema_version": <credential schema version>,
         "issuer_did": <credential issuer did>,
         "cred_def_id": <credential definition id>,
         "rev_reg_id": <credential revocation registry id>, // "None" as string if not present
     
 @param completion Callback that takes command result as parameter.
 Returns
    searchHandle: Search handle that can be used later to fetch records by small batches (with proverFetchCredentialsForProofReqItemReferent)
 */
+ (void)proverSearchCredentialsForProofRequest:(NSString *)proofRequest
                                extraQueryJSON:(NSString *)extraQueryJSON
                                  walletHandle:(IndyHandle)walletHandle
                                    completion:(void (^)(NSError *error, IndyHandle searchHandle))completion;

/**
 Fetch next records for the requested item using proof request search handle (created by proverSearchCredentialsForProofRequest).

 @param searchHandle Search handle (created by proverSearchCredentialsForProofRequest).
 @param itemReferent Referent of attribute/predicate in the proof request.
 @param count Count records to fetch.
 @param completion Callback that takes command result as parameter.
 Returns 
    credentialsJson: List of credentials for the given proof request.
         [{
             cred_info: <credential_info>,
             interval: Optional<non_revoc_interval>
         }]
    where
    credential_info:
         {
            "referent": string, - id of credential in the wallet
            "attrs": {"key1":"raw_value1", "key2":"raw_value2"}, - credential attributes
            "schema_id": string, - identifier of schema
            "cred_def_id": string, - identifier of credential definition
            "rev_reg_id": Optional<string>, - identifier of revocation registry definition
            "cred_rev_id": Optional<string> - identifier of credential in the revocation registry definition
         }
    non_revoc_interval:
         {
             "from": Optional<int>, // timestamp of interval beginning
             "to": Optional<int>, // timestamp of interval ending
         }
      }

    NOTE: The list of length less than the requested count means that search iterator
    correspondent to the requested <itemReferent> is completed.
 */
+ (void)proverFetchCredentialsForProofReqItemReferent:(NSString *)itemReferent
                                         searchHandle:(IndyHandle)searchHandle
                                                count:(NSNumber *)count
                                           completion:(void (^)(NSError *error, NSString *credentialsJson))completion;

/**
 Close credentials search for proof request (make search handle invalid)

 @param searchHandle Search handle (created by proverSearchCredentialsForProofRequest).
 Returns no result

 */
+ (void)proverCloseCredentialsSearchForProofReqWithHandle:(IndyHandle)searchHandle
                                               completion:(void (^)(NSError *error))completion;

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
         "nonce": string, - a decimal number represented as a string (use `indy_generate_nonce` function to generate 80-bit number)
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
                        // (applies to every attribute and predicate but can be overridden on attribute level)
                        // (can be overridden on attribute level)
         "ver": Optional<str>  - proof request version:
            - omit to use unqualified identifiers for restrictions
            - "1.0" to use unqualified identifiers for restrictions
            - "2.0" to use fully qualified identifiers for restrictions
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
         <schema1_id>: <schema1>,
         <schema2_id>: <schema2>,
         <schema3_id>: <schema3>,
     }
 @param credentialDefsJSON: all credential definitions json participating in the proof request
     {
         "cred_def1_id": <credential_def1>,
         "cred_def2_id": <credential_def2>,
         "cred_def3_id": <credential_def3>,
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

  where
  attr_referent: Proof-request local identifier of requested attribute
  attr_info: Describes requested attribute
     {
         "name": Optional<string>, // attribute name, (case insensitive and ignore spaces)
         "names": Optional<[string, string]>, // attribute names, (case insensitive and ignore spaces)
                                              // NOTE: should either be "name" or "names", not both and not none of them.
                                              // Use "names" to specify several attributes that have to match a single credential.
         "restrictions": Optional<filter_json>, // see below
         "non_revoked": Optional<<non_revoc_interval>>, // see below,
                        // If specified prover must proof non-revocation
                        // for date in this interval this attribute
                        // (overrides proof level interval)
     }
  predicate_referent: Proof-request local identifier of requested attribute predicate
  predicate_info: Describes requested attribute predicate
      {
          "name": attribute name, (case insensitive and ignore spaces)
          "p_type": predicate type (">=", ">", "<=", "<")
          "p_value": predicate value
          "restrictions": Optional<wql query>, // see below
          "non_revoked": Optional<<non_revoc_interval>>, // see below,
                         // If specified prover must proof non-revocation
                         // for date in this interval this attribute
                         // (overrides proof level interval)
          "ver": Optional<str>  - proof request version:
            - omit to use unqualified identifiers for restrictions
            - "1.0" to use unqualified identifiers for restrictions
            - "2.0" to use fully qualified identifiers for restrictions
      }
  non_revoc_interval: Defines non-revocation interval
      {
          "from": Optional<int>, // timestamp of interval beginning
          "to": Optional<int>, // timestamp of interval ending
      }
  where wql query: indy-sdk/docs/design/011-wallet-query-language/README.md
      The list of allowed fields:
          "schema_id": <credential schema id>,
          "schema_issuer_did": <credential schema issuer did>,
          "schema_name": <credential schema name>,
          "schema_version": <credential schema version>,
          "issuer_did": <credential issuer did>,
          "cred_def_id": <credential definition id>,
          "rev_reg_id": <credential revocation registry id>, // "None" as string if not present
 
 @param walletHandle Wallet handler (created by IndyWallet::openWalletWithNam).
 
 @param completion Callback that takes command result as parameter. 
  Proof json
  For each requested attribute either a proof (with optionally revealed attribute value) or
  self-attested attribute value is provided.
  Each proof is associated with a credential and corresponding schema_id, cred_def_id, rev_reg_id and timestamp.
  There is also aggregated proof part common for all credential proofs.
     {
         "requested_proof": {
             "revealed_attrs": {
                 "requested_attr1_id": {sub_proof_index: number, raw: string, encoded: string},
                 "requested_attr4_id": {sub_proof_index: number: string, encoded: string},
             },
             "revealed_attr_groups": {
                 "requested_attr5_id": {
                     "sub_proof_index": number,
                     "values": {
                         "attribute_name": {
                             "raw": string,
                             "encoded": string
                         }
                     },
                 }
             },
             "unrevealed_attrs": {
                 "requested_attr3_id": {sub_proof_index: number}
             },
             "self_attested_attrs": {
                 "requested_attr2_id": self_attested_value,
             },
             "predicates": {
                 "requested_predicate_1_referent": {sub_proof_index: int},
                 "requested_predicate_2_referent": {sub_proof_index: int},
             }
         }
         "proof": {
             "proofs": [ <credential_proof>, <credential_proof>, <credential_proof> ],
             "aggregated_proof": <aggregated_proof>
         } (opaque type that contains data structures internal to Ursa.
           It should not be parsed and are likely to change in future versions).
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

 IMPORTANT: You must use *_id's (`schema_id`, `cred_def_id`, `rev_reg_id`) listed in `proof[identifiers]`
 as the keys for corresponding `schemasJSON`, `credentialDefsJSON`, `revocRegDefsJSON`, `revocRegsJSON` objects.

 @param  proofRequestJson: proof request json
     {
         "name": string,
         "version": string,
         "nonce": string, - a decimal number represented as a string (use `indy_generate_nonce` function to generate 80-bit number)
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
         "ver": Optional<str>  - proof request version:
            - omit to use unqualified identifiers for restrictions
            - "1.0" to use unqualified identifiers for restrictions
            - "2.0" to use fully qualified identifiers for restrictions
     }
 @param proofJSON: proof json
     {
         "requested_proof": {
             "revealed_attrs": {
                 "requested_attr1_id": {sub_proof_index: number, raw: string, encoded: string}, // NOTE: check that `encoded` value match to `raw` value on application level
                 "requested_attr4_id": {sub_proof_index: number: string, encoded: string}, // NOTE: check that `encoded` value match to `raw` value on application level
             },
             "revealed_attr_groups": {
                 "requested_attr5_id": {
                     "sub_proof_index": number,
                     "values": {
                         "attribute_name": {
                             "raw": string,
                             "encoded": string
                         }
                     }, // NOTE: check that `encoded` value match to `raw` value on application level
                 }
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
         <schema1_id>: <schema1>,
         <schema2_id>: <schema2>,
         <schema3_id>: <schema3>,
     }
 @param credentialDefsJSON: all credential definitions json participating in the proof request
     {
         "cred_def1_id": <credential_def1>,
         "cred_def2_id": <credential_def2>,
         "cred_def3_id": <credential_def3>,
     }
 @param revocRegDefsJSON: all revocation registry definitions json participating in the proof
     {
         "rev_reg_def1_id": <rev_reg_def1>,
         "rev_reg_def2_id": <rev_reg_def2>,
         "rev_reg_def3_id": <rev_reg_def3>,
     }
 @param revocRegsJSON: all revocation registries json participating in the proof
     {
         "rev_reg_def1_id  or credential_1_id": {
             "timestamp1": <rev_reg1>,
             "timestamp2": <rev_reg2>,
         },
         "rev_reg_def2_id"  or credential_3_id: {
             "timestamp3": <rev_reg3>
         },
         "rev_reg_def3_id  or credential_3_id": {
             "timestamp4": <rev_reg4>
         },
     } - Note: use credential_id instead rev_reg_id in case proving several credentials from the same revocation registry.
     
 where
 attr_referent: Proof-request local identifier of requested attribute
 attr_info: Describes requested attribute
     {
         "name": Optional<string>, // attribute name, (case insensitive and ignore spaces)
         "names": Optional<[string, string]>, // attribute names, (case insensitive and ignore spaces)
                                              // NOTE: should either be "name" or "names", not both and not none of them.
                                              // Use "names" to specify several attributes that have to match a single credential.
         "restrictions": Optional<filter_json>, // see below
         "non_revoked": Optional<<non_revoc_interval>>, // see below,
                        // If specified prover must proof non-revocation
                        // for date in this interval this attribute
                        // (overrides proof level interval)
     }
 predicate_referent: Proof-request local identifier of requested attribute predicate
 predicate_info: Describes requested attribute predicate
     {
         "name": attribute name, (case insensitive and ignore spaces)
         "p_type": predicate type (">=", ">", "<=", "<")
         "p_value": predicate value
         "restrictions": Optional<wql query>, // see below
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
 where wql query: indy-sdk/docs/design/011-wallet-query-language/README.md
     The list of allowed fields:
         "schema_id": <credential schema id>,
         "schema_issuer_did": <credential schema issuer did>,
         "schema_name": <credential schema name>,
         "schema_version": <credential schema version>,
         "issuer_did": <credential issuer did>,
         "cred_def_id": <credential definition id>,
         "rev_reg_id": <credential revocation registry id>, // "None" as string if not present

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
 Create revocation state for a credential that corresponds to a particular time.

 Note that revocation delta must cover the whole registry existence time.
 You can use `from`: `0` and `to`: `needed_time` as parameters for building request to get correct revocation delta.

 The resulting revocation state and provided timestamp can be saved and reused later with applying a new
 revocation delta with `updateRevocationState` function.
 This new delta should be received with parameters: `from`: `timestamp` and `to`: `needed_time`.

 @param  credRevID: user credential revocation id in revocation registry
 @param  timestamp: time represented as a total number of seconds from Unix Epoch
 @param  revRegDefJSON: revocation registry definition json
 @param  revRegDeltaJSON: revocation registry definition delta which covers the whole registry existence time
 @param  blobStorageReaderHandle: configuration of blob storage reader handle that will allow to read revocation tails
 @param completion Callback that takes command result as parameter. 
 Returns result revocation state json:
 {
     "rev_reg": <revocation registry>,
     "witness": <witness>,  (opaque type that contains data structures internal to Ursa.
                             It should not be parsed and are likely to change in future versions).
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
 Create a new revocation state for a credential based on a revocation state created before.
 Note that provided revocation delta must cover the registry gap from based state creation until the specified time
 (this new delta should be received with parameters: `from`: `state_timestamp` and `to`: `needed_time`).

 This function reduces the calculation time.

 The resulting revocation state and provided timestamp can be saved and reused later by applying a new revocation delta again.

 @param  revStateJSON: revocation registry state json
 @param  credRevID: user credential revocation id in revocation registry
 @param  timestamp: time represented as a total number of seconds from Unix Epoch
 @param  revRegDefJSON: revocation registry definition json
 @param  revRegDeltaJSON: revocation registry definition delta which covers the gap form original `rev_state_json` creation till the requested timestamp
 @param  blobStorageReaderHandle: configuration of blob storage reader handle that will allow to read revocation tails
 @param completion Callback that takes command result as parameter.
 Returns result revocation state json:
 {
     "rev_reg": <revocation registry>,
     "witness": <witness>,  (opaque type that contains data structures internal to Ursa.
                             It should not be parsed and are likely to change in future versions).
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

/**
 Generates 80-bit numbers that can be used as a nonce for proof request.

 @param completion Callback that takes command result as parameter.
 Returns nonce: generated number as a string
 */
+ (void)generateNonce:(void (^)(NSError *error, NSString *nonce))completion;

/**
 Get unqualified form (short form without method) of a fully qualified entity like DIDs..

 This function should be used to the proper casting of fully qualified entity to unqualified form in the following cases:
     Issuer, which works with fully qualified identifiers, creates a Credential Offer for Prover, which doesn't support fully qualified identifiers.
     Verifier prepares a Proof Request based on fully qualified identifiers or Prover, which doesn't support fully qualified identifiers.
     another case when casting to unqualified form needed

 @param  entity: utarget entity to disqualify.
    Can be one of:
        Did
        SchemaId
        CredentialDefinitionId
        RevocationRegistryId
        Schema
        CredentialDefinition
        RevocationRegistryDefinition
        CredentialOffer
        CredentialRequest
        ProofRequest

 Returns entity either in unqualified form or original if casting isn't possible
 */
+ (void)toUnqualified:(NSString *)entity
        completion:(void (^)(NSError *error, NSString *res))completion;

@end
