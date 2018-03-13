# Anoncreds Design

Here you can find the requirements and design for Indy SDK Anoncreds workflow (including revocation).

* [Anoncreds References](#anoncreds-references)
* [Design Goals](#design-goals)
* [Anoncreds Workflow](#anoncreds-workflow)
* [API](#api)

## Anoncreds References

Anoncreds protocol links:

* [Anoncreds Workflow](#anoncreds-workflow)
* [Anoncreds Requirements](https://github.com/hyperledger/indy-node/blob/master/design/anoncreds.md#requirements)
* Indy Node Anoncreds transactions:
  * [SCHEMA](https://github.com/hyperledger/indy-node/blob/master/design/anoncreds.md##schema)
  * [CRED_DEF](https://github.com/hyperledger/indy-node/blob/master/design/anoncreds.md##cred_def)
  * [REVOC_REG_DEF](https://github.com/hyperledger/indy-node/blob/master/design/anoncreds.md##revoc_reg_def)
  * [REVOC_REG_ENTRY](https://github.com/hyperledger/indy-node/blob/master/design/anoncreds.md##revoc_reg_entry)
  * [Timestamp Support in State](https://github.com/hyperledger/indy-node/blob/master/design/anoncreds.md#timestamp-support-in-state)
  * [GET_OBJ](https://github.com/hyperledger/indy-node/blob/master/design/anoncreds.md#get_obj)
  * [Issuer Key Rotation](https://github.com/hyperledger/indy-node/blob/master/design/anoncreds.md#issuer-key-rotation)
* [Anoncreds Math](https://github.com/hyperledger/indy-crypto/blob/master/libindy-crypto/docs/AnonCred.pdf)
* [Anoncreds Protocol Crypto API](https://github.com/hyperledger/indy-crypto/blob/master/libindy-crypto/docs/anoncreds-design.md)

## Design Goals

* Indy SDK and Indy Node should use the same format for public Anoncreds entities (Schema, Credential Definition, Revocation Registry Definition, Revocation Registry Delta)
* Indy SDK and Indy Node should use the same entities referencing approach
* It should be possible to integrate additional claim signature and revocation schemas without breaking API changes
* API should provide flexible and pluggable approach to handle revocation tails files
* API should provide the way to calculate revocation witness values on cloud agent to avoid downloading of the hole tails file on edge devices

## Anoncreds Workflow

<img src="./libindy-anoncreds.svg">

## API

### Schema Issuer

```Rust
/// Create credential schema entity that describes credential attributes list and allows credentials
/// interoperability.
///
/// Schema is public and intended to be shared with all anoncreds workflow actors usually by publishing SCHEMA transaction
/// to Indy distributed ledger.
///
/// #Params
/// command_handle: command handle to map callback to user context
/// issuer_did: DID of schema issuer
/// name: a name the schema
/// version: a version of the schema
/// attrs: a list of schema attributes descriptions
/// cb: Callback that takes command result as parameter
///
/// #Returns
/// schema_id: identifier of created schema
/// schema_json: schema as json
///
/// #Errors
/// Common*
/// Anoncreds*
#[no_mangle]
pub extern fn indy_issuer_create_schema(command_handle: i32,
                                        issuer_did: *const c_char,
                                        name: *const c_char,
                                        version: *const c_char,
                                        attrs: *const c_char,
                                        cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                             schema_id: *const c_char
                                                             schema_json: *const c_char)>) -> ErrorCode
```

### Issuer

```Rust
/// Create credential definition entity that encapsulates credentials issuer DID, credential schema, secrets used for signing credentials
/// and secrets used for credentials revocation.
///
/// Credential definition entity contains private and public parts. Private part will be stored in the wallet. Public part
/// will be returned as json intended to be shared with all anoncreds workflow actors usually by publishing CRED_DEF transaction
/// to Indy distributed ledger.
///
/// #Params
/// command_handle: command handle to map callback to user context
/// wallet_handle: wallet handler (created by open_wallet)
/// issuer_did: a DID of the issuer signing claim_def transaction to the Ledger
/// schema_json: credential schema as a json
/// tag: allows to distinct between credential definitions for the same issuer and schema
/// type_: credential definition type (optional, 'CL' by default) that defines claims signature and revocation math. Supported types are:
/// - 'CL': Camenisch-Lysyanskaya credential signature type
/// config_json: type-specific configuration of credential definition as json:
/// - 'CL':
///   - revocationSupport: whether to request non-revocation credential (optional, default false)
/// cb: Callback that takes command result as parameter
///
/// #Returns
/// cred_def_id: identifier of created credential definition
/// cred_def_json: public part of created credential definition
///
/// #Errors
/// Common*
/// Wallet*
/// Anoncreds*
#[no_mangle]
pub extern fn indy_issuer_create_and_store_cred_def(command_handle: i32,
                                                    wallet_handle: i32,
                                                    issuer_did: *const c_char,
                                                    schema_json: *const c_char,
                                                    tag: *const c_char,
                                                    type_: *const c_char,
                                                    config_json: *const c_char,
                                                    cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                         cred_def_id: *const c_char,
                                                                         cred_def_json: *const c_char)>) -> ErrorCode
```

```Rust
/// Create a new revocation registry for the given credential definition as tuple of entities:
/// - Revocation registry definition that encapsulates credentials definition reference, revocation type specific configuration and
///   secrets used for credentials revocation
/// - Revocation registry state that stores the information about revoked entities in a non-disclosing way. The state can be
///   represented as ordered list of revocation registry entries were each entry represents the list of revocation or issuance operations.
///
/// Revocation registry definition entity contains private and public parts. Private part will be stored in the wallet. Public part
/// will be returned as json intended to be shared with all anoncreds workflow actors usually by publishing REVOC_REG_DEF transaction
/// to Indy distributed ledger.
///
/// Revocation registry state is stored on the wallet and also intended to be shared as the ordered list of REVOC_REG_ENTRY transactions.
/// This call initializes the state in the wallet and returns the initial entry.
///
/// Some revocation registry types (for example, 'CL_ACCUM') can require generation of binary blob called tails used to hide information about revoked claims in public
/// revocation registry and intended to be distributed out of leger (REVOC_REG_DEF transaction will still contain uri and hash of tails).
/// This call requires access to pre-configured blob storage writer instance handle that will allow to write generated tails.
///
/// #Params
/// command_handle: command handle to map callback to user context
/// wallet_handle: wallet handler (created by open_wallet)
/// blob_storage_writer_handle: pre-configured blob storage writer instance handle that will allow to write generated tails
/// type_: revocation registry type (optional, default value depends on claim definition type). Supported types are:
/// - 'CL_ACCUM': Type-3 pairing based accumulator. Default for 'CL' claim definition type
/// config_json: type-specific configuration of revocation registry as json:
/// - 'CL_ACCUM':
///   - maxClaimsNum: maximum number of claims the new registry can process (optional, default 100000)
///   - issuanceByDefault: issuance type (optional, default true). If:
///       - true: all indices are assumed to be issued and initial accumulator is calculated over all indices; Revocation registry is updated only during revocation
///       - false: nothing is issued initially accumulator is 1
/// cb: Callback that takes command result as parameter
///
/// #Returns
/// revoc_reg_def_id: identifer of created revocation registry definition
/// revoc_reg_def_json: public part of revocation registry definition
/// revoc_reg_entry_json:revocation registry entry that defines initial state of revocation registry
///
/// #Errors
/// Common*
/// Wallet*
/// Anoncreds*
#[no_mangle]
pub extern fn indy_issuer_create_and_store_revoc_reg(command_handle: i32,
                                                     wallet_handle: i32,
                                                     blob_storage_writer_handle: i32,
                                                     cred_def_id:  *const c_char,
                                                     tag: *const c_char,
                                                     type_: *const c_char,
                                                     config_json: *const c_char,
                                                     cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                          revoc_reg_def_id: *const c_char,
                                                                          revoc_reg_def_json: *const c_char,
                                                                          revoc_reg_entry_json: *const c_char)>) -> ErrorCode
```

```Rust
/// Create credential offer that will be used by Prover for
/// claim request creation. Offer includes nonce and key correctness proof
/// for authentication between protocol steps and integrity checking.
///
/// #Params
/// command_handle: command handle to map callback to user context
/// wallet_handle: wallet handler (created by open_wallet)
/// cred_def_id: id of credential definition stored in the wallet
/// rev_reg_id: id of revocation registry definition stored in the wallet
/// cb: Callback that takes command result as parameter
///
/// #Returns
/// credential offer json:
///   {
///     "cred_def_id": string,
///     "rev_reg_def_id" : Optional<string>,
///     // Fields below can depend on Cred Def type
///     "nonce": string,
///     "key_correctness_proof" : <key_correctness_proof>
///   }
///
/// #Errors
/// Common*
/// Wallet*
/// Anoncreds*
#[no_mangle]
pub extern fn indy_issuer_create_cred_offer(command_handle: i32,
                                            wallet_handle: i32,
                                            cred_def_id: *const c_char,
                                            rev_reg_def_id: *const c_char,
                                            cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                 claim_offer_json: *const c_char)>) -> ErrorCode
```

```Rust
/// Check Cred Request for the given Cred Offer and issue Credential for the given Cred Request.
///
/// Cred Request must match Cred Offer. The credential definition and revocation registry definition
/// referenced in Cred Offer and Cred Request
/// must be already created and stored into the wallet.
///
/// Information for this credential revocation will be store in the wallet as part of revocation registry under
/// generated cred_revoc_id local for this wallet.
///
/// This call returns revoc registry delta as json file intended to be shared as REVOC_REG_ENTRY transaction.
/// Note that it is possible to accumulate deltas to reduce ledger load.
///
/// #Params
/// command_handle: command handle to map callback to user context.
/// wallet_handle: wallet handler (created by open_wallet).
/// blob_storage_reader_handle: pre-configured blob storage reader instance handle that will allow to read revocation tails
/// cred_offer_json: a cred offer created by indy_issuer_create_cred_offer
/// cred_req_json: a credential request created by indy_prover_create_cred_request
/// cred_values_json: a credential containing attribute values for each of requested attribute names.
///     Example:
///     {
///      "attr1" : {"raw": "value1", "encoded": "value1_as_int" },
///      "attr2" : {"raw": "value1", "encoded": "value1_as_int" }
///     }
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// revoc_id: local id for revocation info (Can be used for revocation of this cred)
/// revoc_reg_delta_json: Revocation registry update json with a newly issued credential
/// cred_json: Credential json containing signed credential values
///     {
///         "cred_def_id": string,
///         "rev_reg_def_id", Optional<string>,
///         "values": <see credential_values_json above>,
///         "signature": <signature>,
///         "signature_correctness_proof": <signature_correctness_proof>,
///         "revoc_idx": // TODO: FIXME: Think how to share it in a secure way
///     }
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn indy_issuer_create_cred(command_handle: i32,
                                      wallet_handle: i32,
                                      blob_storage_reader_handle: i32,
                                      cred_offer_json: *const c_char,
                                      cred_req_json: *const c_char,
                                      cred_values_json: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                           cred_revoc_id: *const c_char,
                                                           revoc_reg_delta_json: *const c_char,
                                                           cred_json: *const c_char)>) -> ErrorCode
```

```Rust
/// Revoke a credential identified by a cred_revoc_id (returned by indy_issuer_create_cred).
///
/// The corresponding credential definition and revocation registry must be already
/// created an stored into the wallet.
///
/// This call returns revoc registry delta as json file intended to be shared as REVOC_REG_ENTRY transaction.
/// Note that it is possible to accumulate deltas to reduce ledger load.
///
/// #Params
/// command_handle: command handle to map callback to user context.
/// wallet_handle: wallet handler (created by open_wallet).
/// blob_storage_reader_handle: pre-configured blob storage reader instance handle that will allow to read revocation tails
/// rev_reg_id: id of revocation registry stored in wallet
/// tails_reader_handle:
/// user_revoc_index: index of the user in the revocation registry
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// revoc_reg_delta_json: Revocation registry delta json with a revoked credential
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn indy_issuer_revoke_cred(command_handle: i32,
                                      wallet_handle: i32,
                                      blob_storage_reader_handle: i32,
                                      cred_revoc_id: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                           revoc_reg_delta_json: *const c_char)>) -> ErrorCode
```

```Rust
/// Recover a credential identified by a cred_revoc_id (returned by indy_issuer_create_cred).
///
/// The corresponding credential definition and revocation registry must be already
/// created an stored into the wallet.
/// 
/// This call returns revoc registry delta as json file intended to be shared as REVOC_REG_ENTRY transaction.
/// Note that it is possible to accumulate deltas to reduce ledger load.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// rev_reg_id: id of revocation registry stored in wallet
/// tails_reader_handle:
/// user_revoc_index: index of the user in the revocation registry
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// revoc_reg_delta_json: Revocation registry delta json with a revoked credential
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn indy_issuer_recover_credential(command_handle: i32,
                                             wallet_handle: i32,
                                             blob_storage_reader_handle: i32,
                                             cred_revoc_id: *const c_char,
                                             cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                  revoc_reg_delta_json: *const c_char)>) -> ErrorCode
```

### Prover

```Rust
/// Creates a master secret and stores it in the wallet.
///
/// #Params
/// command_handle: command handle to map callback to user context.
/// wallet_handle: wallet handler (created by open_wallet).
/// master_secret_id: (optional, if not present random one will be generated) new master id
///
/// #Returns
/// master_secret_id: Id of generated master secret
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn indy_prover_create_master_secret(command_handle: i32,
                                               wallet_handle: i32,
                                               master_secret_id: *const c_char,
                                               cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                    master_secret_id: *const c_char)>) -> ErrorCode
```

```Rust
/// Creates a clam request for the given credential offer.
///
/// The method creates a blinded master secret for a master secret identified by a provided name.
/// The master secret identified by the name must be already stored in the secure wallet (see prover_create_master_secret)
/// The blinded master secret is a part of the credential request.
///
/// #Params
/// command_handle: command handle to map callback to user context
/// wallet_handle: wallet handler (created by open_wallet)
/// prover_did: a DID of the prover
/// cred_offer_json: a cred offer created by indy_issuer_create_cred_offer
/// cred_def_json: credential definition json created by indy_issuer_create_and_store_cred_def
/// master_secret_id: the id of the master secret stored in the wallet
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// cred_req_json: Credential request json
///     {
///      "cred_def_id" : string,
///      "rev_reg_id" : Optional<string>,
///      "prover_did" : string,
///       // Fields below are depend on anoncreds crypto type
///      "blinded_ms" : <blinded_master_secret>,
///      "blinded_ms_correctness_proof" : <blinded_ms_correctness_proof>,
///      "nonce": string
///    }
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn indy_prover_create_cred_req(command_handle: i32,
                                          wallet_handle: i32,
                                          prover_did: *const c_char,
                                          cred_offer_json: *const c_char,
                                          cred_def_json: *const c_char,
                                          master_secret_id: *const c_char,
                                          cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                cred_req_json: *const c_char)>) -> ErrorCode
```

```Rust
/// Check credential provided by Issuer for the given credential request,
/// updates the credential by a master secret and stores in a secure wallet.
///
///
/// #Params
/// command_handle: command handle to map callback to user context.
/// wallet_handle: wallet handler (created by open_wallet).
/// id: (optional, default is a random one) identifier by which credential will be stored in the wallet
/// cred_req_json: a credential request created by indy_prover_create_cred_request
/// cred_json: credential json created by indy_issuer_create_cred
/// schema_json: a schema that was used for credential issuance
/// cred_def_json: credential definition json created by indy_issuer_create_and_store_cred_def
/// rev_reg_def_json: revocation registry definition json created by indy_issuer_create_and_store_revoc_reg
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// cred_id: identifier by which credential is stored in the wallet
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn indy_prover_store_cred(command_handle: i32,
                                     wallet_handle: i32,
                                     cred_id: *const c_char,
                                     cred_req_json: *const c_char,
                                     cred_json: *const c_char,
                                     cred_schema_json: *const c_char,
                                     cred_def_json: *const c_char,
                                     rev_reg_def_json: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                          cred_id: *const c_char)>) -> ErrorCode
```

### Blob Storage

CL revocation schema introduces Revocation Tails entity used to hide information about revoked claims in public Revocation Registry. Tails

* are static (once generated) array of BigIntegers that can be represented as binary blob or file
* may require quite huge amount of data (up to 1GB per Revocation Registry);
* are created and shared by Issuers;
* are required (so must be available for download) for both Provers and Issuers;
* can be cached and can be downloaded only once;
* Some operation (incremental witness updates) can require reading only small part of blob file. It can be more efficient to store complete tails blob in the cloud and ask for small parts through network.

As result the way how to access tails blobs can be very application specific. To address this SDK will provide the following:

* API for registering custom handler for blobs reading
* API for registering custom handler for blobs writing
* API for blob consistency validation
* Default handlers implementation that will allow to read blobs from local file and write blobs to local file.

Tails publishing and access workflow can be integrated with Indy Node in the following way:

* Issuer generates tails and writes tails blob to local file (with default handler). Our API will also provide blob hash to him and generate URI based on configurable URI pattern.
* Issuer uploads blob to some CDN with corresponded URI (out of SDK scope)
* Issuer sends REVOC_REG_DEF transaction with and publishes tails URI and hash
* Prover sends GET_REVOC_REG_DEF requests and receives tails URI and hash
* Prover downloads published tails file and stores it locally (Out of SDK)