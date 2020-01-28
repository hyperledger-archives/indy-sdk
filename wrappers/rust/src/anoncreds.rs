use {ErrorCode, IndyError};

use std::ffi::CString;
use std::ptr::null;

use futures::Future;

use utils::callbacks::{ClosureHandler, ResultHandler};

use ffi::anoncreds;
use ffi::{ResponseStringStringCB,
          ResponseI32UsizeCB,
          ResponseStringStringStringCB,
          ResponseStringCB,
          ResponseI32CB,
          ResponseEmptyCB,
          ResponseBoolCB};
use {CommandHandle, WalletHandle, SearchHandle, BlobStorageReaderHandle, TailsWriterHandle};
use ffi::BlobStorageReaderCfgHandle;

/*
These functions wrap the Ursa algorithm as documented in this paper:
https://github.com/hyperledger/ursa/blob/master/libursa/docs/AnonCred.pdf

And is documented in this HIPE:
https://github.com/hyperledger/indy-hipe/blob/c761c583b1e01c1e9d3ceda2b03b35336fdc8cc1/text/anoncreds-protocol/README.md
*/


/// Create credential schema entity that describes credential attributes list and allows credentials
/// interoperability.
///
/// Schema is public and intended to be shared with all anoncreds workflow actors usually by publishing SCHEMA transaction
/// to Indy distributed ledger.
///
/// It is IMPORTANT for current version POST Schema in Ledger and after that GET it from Ledger
/// with correct seq_no to save compatibility with Ledger.
/// After that can call create_and_store_credential_def to build corresponding Credential Definition.
///
/// # Arguments
/// * `pool_handle` - pool handle (created by Pool::open_ledger).
/// * `issuer_did`: DID of schema issuer
/// * `name`: a name the schema
/// * `version`: a version of the schema
/// * `attrs`: a list of schema attributes descriptions (the number of attributes should be less or equal than 125)
///
/// # Returns
/// * `schema_id`: identifier of created schema
/// * `schema_json`: schema as json
pub fn issuer_create_schema(issuer_did: &str, name: &str, version: &str, attrs: &str) -> Box<dyn Future<Item=(String, String), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

    let err = _issuer_create_schema(command_handle, issuer_did, name, version, attrs, cb);

    ResultHandler::str_str(command_handle, err, receiver)
}

fn _issuer_create_schema(command_handle: CommandHandle, issuer_did: &str, name: &str, version: &str, attrs: &str, cb: Option<ResponseStringStringCB>) -> ErrorCode {
    let issuer_did = c_str!(issuer_did);
    let name = c_str!(name);
    let version = c_str!(version);
    let attrs = c_str!(attrs);

    ErrorCode::from(unsafe {
        anoncreds::indy_issuer_create_schema(command_handle, issuer_did.as_ptr(), name.as_ptr(), version.as_ptr(), attrs.as_ptr(), cb)
    })
}

/// Create credential definition entity that encapsulates credentials issuer DID, credential schema, secrets used for signing credentials
/// and secrets used for credentials revocation.
///
/// Credential definition entity contains private and public parts. Private part will be stored in the wallet. Public part
/// will be returned as json intended to be shared with all anoncreds workflow actors usually by publishing CRED_DEF transaction
/// to Indy distributed ledger.
///
/// It is IMPORTANT for current version GET Schema from Ledger with correct seq_no to save compatibility with Ledger.
///
/// Note: Use combination of `issuer_rotate_credential_def_start` and `issuer_rotate_credential_def_apply` functions
/// to generate new keys for an existing credential definition.
///
/// # Arguments
/// * `wallet_handle`: wallet handle (created by Wallet::open_wallet).
/// * `issuer_did`: a DID of the issuer signing cred_def transaction to the Ledger
/// * `schema_json`: credential schema as a json
/// * `tag`: allows to distinct between credential definitions for the same issuer and schema
/// * `signature_type`: credential definition type (optional, 'CL' by default) that defines credentials signature and revocation math. Supported types are:
///     - 'CL': Camenisch-Lysyanskaya credential signature type that is implemented according to the algorithm in this paper:
///             https://github.com/hyperledger/ursa/blob/master/libursa/docs/AnonCred.pdf
///         And is documented in this HIPE:
///             https://github.com/hyperledger/indy-hipe/blob/c761c583b1e01c1e9d3ceda2b03b35336fdc8cc1/text/anoncreds-protocol/README.md
/// * `config_json`: (optional) type-specific configuration of credential definition as json:
///     - 'CL':
///         - support_revocation: whether to request non-revocation credential (optional, default false)
///
/// # Returns
/// * `cred_def_id`: identifier of created credential definition
/// * `cred_def_json`: public part of created credential definition
/// {
///     id: string - identifier of credential definition
///     schemaId: string - identifier of stored in ledger schema
///     type: string - type of the credential definition. CL is the only supported type now.
///     tag: string - allows to distinct between credential definitions for the same issuer and schema
///     value: Dictionary with Credential Definition's data is depended on the signature type: {
///         primary: primary credential public key,
///         Optional<revocation>: revocation credential public key
///     },
///     ver: Version of the CredDef json
/// }
///
/// Note: `primary` and `revocation` fields of credential definition are complex opaque types that contain data structures internal to Ursa.
/// They should not be parsed and are likely to change in future versions.
///
pub fn issuer_create_and_store_credential_def(wallet_handle: WalletHandle, issuer_did: &str, schema_json: &str, tag: &str, signature_type: Option<&str>, config_json: &str) -> Box<dyn Future<Item=(String, String), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

    let err = _issuer_create_and_store_credential_def(command_handle, wallet_handle, issuer_did, schema_json, tag, signature_type, config_json, cb);

    ResultHandler::str_str(command_handle, err, receiver)
}

fn _issuer_create_and_store_credential_def(command_handle: CommandHandle, wallet_handle: WalletHandle, issuer_did: &str, schema_json: &str, tag: &str, signature_type: Option<&str>, config_json: &str, cb: Option<ResponseStringStringCB>) -> ErrorCode {
    let issuer_did = c_str!(issuer_did);
    let schema_json = c_str!(schema_json);
    let tag = c_str!(tag);
    let signature_type_str = opt_c_str!(signature_type);
    let config_json = c_str!(config_json);

    ErrorCode::from(unsafe {
        anoncreds::indy_issuer_create_and_store_credential_def(
            command_handle,
            wallet_handle,
            issuer_did.as_ptr(),
            schema_json.as_ptr(),
            tag.as_ptr(),
            opt_c_ptr!(signature_type, signature_type_str),
            config_json.as_ptr(),
            cb
        )
    })
}

/// Generate temporary credential definitional keys for an existing one (owned by the caller of the library).
///
/// Use `issuer_rotate_credential_def_apply` function to set generated temporary keys as the main.
///
/// # Arguments
/// * `wallet_handle`: wallet handle (created by Wallet::open_wallet).
/// * `cred_def_id`: an identifier of created credential definition stored in the wallet
/// * `config_json`: (optional) type-specific configuration of credential definition as json:
///     - 'CL':
///         - support_revocation: whether to request non-revocation credential (optional, default false)
///
/// # Returns
/// * `cred_def_json`: public part of temporary created credential definition
pub fn issuer_rotate_credential_def_start(wallet_handle: WalletHandle, cred_def_id: &str, config_json: Option<&str>) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _issuer_rotate_credential_def_start(command_handle, wallet_handle, cred_def_id, config_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _issuer_rotate_credential_def_start(command_handle: CommandHandle, wallet_handle: WalletHandle, cred_def_id: &str, config: Option<&str>, cb: Option<ResponseStringCB>) -> ErrorCode {
    let cred_def_id = c_str!(cred_def_id);
    let config_str = opt_c_str!(config);

    ErrorCode::from(unsafe {
        anoncreds::indy_issuer_rotate_credential_def_start(
            command_handle,
            wallet_handle,
            cred_def_id.as_ptr(),
            opt_c_ptr!(config, config_str),
            cb
        )
    })
}

/// Apply temporary keys as main for an existing Credential Definition (owned by the caller of the library).
///
/// # Arguments
/// * `wallet_handle`: wallet handle (created by Wallet::open_wallet).
/// * `cred_def_id`: an identifier of created credential definition stored in the wallet
///
/// # Returns
pub fn issuer_rotate_credential_def_apply(wallet_handle: WalletHandle, cred_def_id: &str) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _issuer_rotate_credential_def_apply(command_handle, wallet_handle, cred_def_id, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _issuer_rotate_credential_def_apply(command_handle: CommandHandle, wallet_handle: WalletHandle, cred_def_id: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
    let cred_def_id = c_str!(cred_def_id);

    ErrorCode::from(unsafe {
        anoncreds::indy_issuer_rotate_credential_def_apply(
            command_handle,
            wallet_handle,
            cred_def_id.as_ptr(),
            cb
        )
    })
}

/// Create a new revocation registry for the given credential definition as tuple of entities
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
/// Some revocation registry types (for example, 'CL_ACCUM') can require generation of binary blob called tails used to hide information about revoked credentials in public
/// revocation registry and intended to be distributed out of leger (REVOC_REG_DEF transaction will still contain uri and hash of tails).
/// This call requires access to pre-configured blob storage writer instance handle that will allow to write generated tails.
///
/// # Arguments
/// * `wallet_handle`: wallet handle (created by Wallet::open_wallet).
/// * `issuer_did`: a DID of the issuer signing transaction to the Ledger
/// * `revoc_def_type`: revocation registry type (optional, default value depends on credential definition type). Supported types are:
/// - 'CL_ACCUM': Type-3 pairing based accumulator implemented according to the algorithm in this paper:
///                   https://github.com/hyperledger/ursa/blob/master/libursa/docs/AnonCred.pdf
///               This type is default for 'CL' credential definition type./// * `tag`: allows to distinct between revocation registries for the same issuer and credential definition
/// * `cred_def_id`: id of stored in ledger credential definition
/// * `config_json`: type-specific configuration of revocation registry as json:
///     - 'CL_ACCUM': {
///         "issuance_type": (optional) type of issuance. Currently supported:
///             1) ISSUANCE_BY_DEFAULT: all indices are assumed to be issued and initial accumulator is calculated over all indices;
///             Revocation Registry is updated only during revocation.
///             2) ISSUANCE_ON_DEMAND: nothing is issued initially accumulator is 1 (used by default);
///         "max_cred_num": maximum number of credentials the new registry can process (optional, default 100000)
///     }
/// * `tails_writer_handle`: handle of blob storage to store tails
///
/// NOTE:
///     Recursive creation of folder for Default Tails Writer (correspondent to `tails_writer_handle`)
///     in the system-wide temporary directory may fail in some setup due to permissions: `IO error: Permission denied`.
///     In this case use `TMPDIR` environment variable to define temporary directory specific for an application.
///
/// # Returns
/// * `revoc_reg_id`: identifier of created revocation registry definition
/// * `revoc_reg_def_json`: public part of revocation registry definition
///     {
///         "id": string - ID of the Revocation Registry,
///         "revocDefType": string - Revocation Registry type (only CL_ACCUM is supported for now),
///         "tag": string - Unique descriptive ID of the Registry,
///         "credDefId": string - ID of the corresponding CredentialDefinition,
///         "value": Registry-specific data {
///             "issuanceType": string - Type of Issuance(ISSUANCE_BY_DEFAULT or ISSUANCE_ON_DEMAND),
///             "maxCredNum": number - Maximum number of credentials the Registry can serve.
///             "tailsHash": string - Hash of tails.
///             "tailsLocation": string - Location of tails file.
///             "publicKeys": <public_keys> - Registry's public key (opaque type that contains data structures internal to Ursa.
///                                                                  It should not be parsed and are likely to change in future versions).
///         },
///         "ver": string - version of revocation registry definition json.
///     }
/// * `revoc_reg_entry_json`: revocation registry entry that defines initial state of revocation registry
/// {
///     value: {
///         prevAccum: string - previous accumulator value.
///         accum: string - current accumulator value.
///         issued: array<number> - an array of issued indices.
///         revoked: array<number> an array of revoked indices.
///     },
///     ver: string - version revocation registry entry json
/// }
pub fn issuer_create_and_store_revoc_reg(wallet_handle: WalletHandle,
                                         issuer_did: &str,
                                         revoc_def_type: Option<&str>,
                                         tag: &str,
                                         cred_def_id: &str,
                                         config_json: &str,
                                         tails_writer_handle: TailsWriterHandle) -> Box<dyn Future<Item=(String, String, String), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string_string();

    let err = _issuer_create_and_store_revoc_reg(command_handle, wallet_handle, issuer_did, revoc_def_type, tag, cred_def_id, config_json, tails_writer_handle, cb);

    ResultHandler::str_str_str(command_handle, err, receiver)
}

fn _issuer_create_and_store_revoc_reg(command_handle: CommandHandle, wallet_handle: WalletHandle, issuer_did: &str, revoc_def_type: Option<&str>, tag: &str, cred_def_id: &str, config_json: &str, tails_writer_handle: TailsWriterHandle, cb: Option<ResponseStringStringStringCB>) -> ErrorCode {
    let issuer_did = c_str!(issuer_did);
    let revoc_def_type_str = opt_c_str!(revoc_def_type);
    let tag = c_str!(tag);
    let cred_def_id = c_str!(cred_def_id);
    let config_json = c_str!(config_json);

    ErrorCode::from(unsafe {
        anoncreds::indy_issuer_create_and_store_revoc_reg(command_handle, wallet_handle, issuer_did.as_ptr(), opt_c_ptr!(revoc_def_type, revoc_def_type_str), tag.as_ptr(), cred_def_id.as_ptr(), config_json.as_ptr(), tails_writer_handle, cb)
    })
}

/// Create credential offer that will be used by Prover for
/// credential request creation. Offer includes nonce and key correctness proof
/// for authentication between protocol steps and integrity checking.
///
/// # Arguments
/// * `wallet_handle`: wallet handle (created by Wallet::open_wallet)
/// * `cred_def_id`: id of credential definition stored in the wallet
///
/// # Returns
/// * `credential_offer_json` - {
///     "schema_id": string,
///     "cred_def_id": string,
///     // Fields below can depend on Cred Def type
///     "nonce": string,
///     "key_correctness_proof" : key correctness proof for credential definition correspondent to cred_def_id
///                                   (opaque type that contains data structures internal to Ursa.
///                                   It should not be parsed and are likely to change in future versions).
/// }
pub fn issuer_create_credential_offer(wallet_handle: WalletHandle, cred_def_id: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _issuer_create_credential_offer(command_handle, wallet_handle, cred_def_id, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _issuer_create_credential_offer(command_handle: CommandHandle, wallet_handle: WalletHandle, cred_def_id: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let cred_def_id = c_str!(cred_def_id);

    ErrorCode::from(unsafe {
        anoncreds::indy_issuer_create_credential_offer(command_handle, wallet_handle, cred_def_id.as_ptr(), cb)
    })
}

/// Check Cred Request for the given Cred Offer and issue Credential for the given Cred Request.
///
/// Cred Request must match Cred Offer. The credential definition and revocation registry definition
/// referenced in Cred Offer and Cred Request must be already created and stored into the wallet.
///
/// Information for this credential revocation will be store in the wallet as part of revocation registry under
/// generated cred_revoc_id local for this wallet.
///
/// This call returns revoc registry delta as json file intended to be shared as REVOC_REG_ENTRY transaction.
/// Note that it is possible to accumulate deltas to reduce ledger load.
///
/// # Arguments
/// * `wallet_handle`: wallet handle (created by Wallet::open_wallet).
/// * `cred_offer_json`: a cred offer created by create_credential_offer
/// * `cred_req_json`: a credential request created by store_credential
/// * `cred_values_json`: a credential containing attribute values for each of requested attribute names.
///     Example:
///     {
///      "attr1" : {"raw": "value1", "encoded": "value1_as_int" },
///      "attr2" : {"raw": "value1", "encoded": "value1_as_int" }
///     }
///    If you want to use empty value for some credential field, you should set "raw" to "" and "encoded" should not be empty
/// * `rev_reg_id`: id of revocation registry stored in the wallet
/// * `blob_storage_reader_handle`: configuration of blob storage reader handle that will allow to read revocation tails
///
/// # Returns
/// * `cred_json`: Credential json containing signed credential values
///     {
///         "schema_id": string,
///         "cred_def_id": string,
///         "rev_reg_def_id", Optional<string>,
///         "values": <see cred_values_json above>,
///         // Fields below can depend on Cred Def type
///         "signature": <credential signature>,
///                      (opaque type that contains data structures internal to Ursa.
///                       It should not be parsed and are likely to change in future versions).
///         "signature_correctness_proof": credential signature correctness proof
///                      (opaque type that contains data structures internal to Ursa.
///                       It should not be parsed and are likely to change in future versions).
///     }
/// * `cred_revoc_id`: local id for revocation info (Can be used for revocation of this credential)
/// * `revoc_reg_delta_json`: Revocation registry delta json with a newly issued credential
pub fn issuer_create_credential(wallet_handle: WalletHandle,
                                cred_offer_json: &str,
                                cred_req_json: &str,
                                cred_values_json: &str,
                                rev_reg_id: Option<&str>,
                                blob_storage_reader_handle: BlobStorageReaderHandle) -> Box<dyn Future<Item=(String, Option<String>, Option<String>), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_opt_string_opt_string();

    let err = _issuer_create_credential(command_handle, wallet_handle, cred_offer_json, cred_req_json, cred_values_json, rev_reg_id, blob_storage_reader_handle, cb);

    ResultHandler::str_optstr_optstr(command_handle, err, receiver)
}

fn _issuer_create_credential(
    command_handle: CommandHandle,
    wallet_handle: WalletHandle,
    cred_offer_json: &str,
    cred_req_json: &str,
    cred_values_json: &str,
    rev_reg_id: Option<&str>,
    blob_storage_reader_handle: BlobStorageReaderHandle,
    cb: Option<ResponseStringStringStringCB>
) -> ErrorCode {
    let cred_offer_json = c_str!(cred_offer_json);
    let cred_req_json = c_str!(cred_req_json);
    let cred_values_json = c_str!(cred_values_json);
    let rev_reg_id_str = opt_c_str!(rev_reg_id);

    ErrorCode::from(unsafe {
        anoncreds::indy_issuer_create_credential(command_handle, wallet_handle, cred_offer_json.as_ptr(), cred_req_json.as_ptr(), cred_values_json.as_ptr(), opt_c_ptr!(rev_reg_id, rev_reg_id_str), blob_storage_reader_handle, cb)
    })
}

/// Revoke a credential identified by a cred_revoc_id (returned by indy_issuer_create_credential).
///
/// The corresponding credential definition and revocation registry must be already
/// created an stored into the wallet.
///
/// This call returns revoc registry delta as json file intended to be shared as REVOC_REG_ENTRY transaction.
/// Note that it is possible to accumulate deltas to reduce ledger load.
///
/// # Arguments
/// * `wallet_handle`: wallet handle (created by Wallet::open_wallet).
/// * `blob_storage_reader_cfg_handle`: configuration of blob storage reader handle that will allow to read revocation tails
/// * `rev_reg_id: id of revocation` registry stored in wallet
/// * `cred_revoc_id`: local id for revocation info
///
/// # Returns
/// * `revoc_reg_delta_json`: Revocation registry delta json with a revoked credential
pub fn issuer_revoke_credential(wallet_handle: WalletHandle, blob_storage_reader_cfg_handle: BlobStorageReaderCfgHandle, rev_reg_id: &str, cred_revoc_id: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _issuer_revoke_credential(command_handle, wallet_handle, blob_storage_reader_cfg_handle, rev_reg_id, cred_revoc_id, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _issuer_revoke_credential(command_handle: CommandHandle,
                             wallet_handle: WalletHandle,
                             blob_storage_reader_cfg_handle: BlobStorageReaderCfgHandle,
                             rev_reg_id: &str,
                             cred_revoc_id: &str,
                             cb: Option<ResponseStringCB>) -> ErrorCode {
    let rev_reg_id = c_str!(rev_reg_id);
    let cred_revoc_id = c_str!(cred_revoc_id);

    ErrorCode::from(unsafe {
        anoncreds::indy_issuer_revoke_credential(command_handle, wallet_handle, blob_storage_reader_cfg_handle, rev_reg_id.as_ptr(), cred_revoc_id.as_ptr(), cb)
    })
}

/// Merge two revocation registry deltas (returned by create_credential or revoke_credential) to accumulate common delta.
/// Send common delta to ledger to reduce the load.
///
/// # Arguments
/// * `rev_reg_delta_json`: revocation registry delta.
/// * `other_rev_reg_delta_json`: revocation registry delta for which PrevAccum value  is equal to current accum value of rev_reg_delta_json.
///
/// # Returns
/// * `merged_rev_reg_delta` - Merged revocation registry delta
pub fn issuer_merge_revocation_registry_deltas(rev_reg_delta_json: &str, other_rev_reg_delta_json: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _issuer_merge_revocation_registry_deltas(command_handle, rev_reg_delta_json, other_rev_reg_delta_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _issuer_merge_revocation_registry_deltas(command_handle: CommandHandle, rev_reg_delta_json: &str, other_rev_reg_delta_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let rev_reg_delta_json = c_str!(rev_reg_delta_json);
    let other_rev_reg_delta_json = c_str!(other_rev_reg_delta_json);

    ErrorCode::from(unsafe {
        anoncreds::indy_issuer_merge_revocation_registry_deltas(command_handle, rev_reg_delta_json.as_ptr(), other_rev_reg_delta_json.as_ptr(), cb)
    })
}


/// Creates a master secret with a given id and stores it in the wallet.
/// The id must be unique.
///
/// # Arguments
/// * `wallet_handle`: wallet handle (created by Wallet::open_wallet).
/// * `master_secret_id`: (optional, if not present random one will be generated) new master id
///
/// # Returns
/// * `out_master_secret_id` - Id of generated master secret
pub fn prover_create_master_secret(wallet_handle: WalletHandle, master_secret_id: Option<&str>) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _prover_create_master_secret(command_handle, wallet_handle, master_secret_id, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _prover_create_master_secret(command_handle: CommandHandle, wallet_handle: WalletHandle, master_secret_id: Option<&str>, cb: Option<ResponseStringCB>) -> ErrorCode {
    let master_secret_id_str = opt_c_str!(master_secret_id);

    ErrorCode::from(unsafe {
        anoncreds::indy_prover_create_master_secret(command_handle, wallet_handle, opt_c_ptr!(master_secret_id, master_secret_id_str), cb)
    })
}

/// Gets human readable credential by the given id.
///
/// # Arguments
/// * `wallet_handle`: wallet handle (created by Wallet::open_wallet).
/// * `cred_id`: Identifier by which requested credential is stored in the wallet
///
/// # Returns
/// * `credential_json` - {
///     "referent": string, // cred_id in the wallet
///     "attrs": {"key1":"raw_value1", "key2":"raw_value2"},
///     "schema_id": string,
///     "cred_def_id": string,
///     "rev_reg_id": Optional<string>,
///     "cred_rev_id": Optional<string>
/// }
pub fn prover_get_credential(wallet_handle: WalletHandle, cred_id: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _prover_get_credential(command_handle, wallet_handle, cred_id, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _prover_get_credential(command_handle: CommandHandle, wallet_handle: WalletHandle, cred_id: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let cred_id = c_str!(cred_id);

    ErrorCode::from(unsafe {
        anoncreds::indy_prover_get_credential(command_handle, wallet_handle, cred_id.as_ptr(), cb)
    })
}

/// Deletes credential by given id.
///
/// # Arguments
/// * `wallet_handle`: wallet handle (created by Wallet::open_wallet).
/// * `cred_id`: Identifier by which requested credential is stored in the wallet
pub fn prover_delete_credential(wallet_handle: WalletHandle, cred_id: &str) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _prover_delete_credential(command_handle, wallet_handle, cred_id, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _prover_delete_credential(command_handle: CommandHandle, wallet_handle: WalletHandle, cred_id: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
    let cred_id = c_str!(cred_id);

    ErrorCode::from(unsafe {
        anoncreds::indy_prover_delete_credential(command_handle, wallet_handle, cred_id.as_ptr(), cb)
    })
}

/// Creates a credential request for the given credential offer.
///
/// The method creates a blinded master secret for a master secret identified by a provided name.
/// The master secret identified by the name must be already stored in the secure wallet (see create_master_secret)
/// The blinded master secret is a part of the credential request.
///
/// # Arguments
/// * `wallet_handle`: wallet handle (created by open_wallet)
/// * `prover_did`: a DID of the prover
/// * `cred_offer_json`: credential offer as a json containing information about the issuer and a credential
/// * `cred_def_json`: credential definition json related to <cred_def_id> in <cred_offer_json>
/// * `master_secret_id`: the id of the master secret stored in the wallet
///
/// # Returns
/// * `cred_req_json`: Credential request json for creation of credential by Issuer
///     {
///      "prover_did" : string,
///      "cred_def_id" : string,
///         // Fields below can depend on Cred Def type
///      "blinded_ms" : <blinded_master_secret>,
///                     (opaque type that contains data structures internal to Ursa.
///                      It should not be parsed and are likely to change in future versions).
///      "blinded_ms_correctness_proof" : <blinded_ms_correctness_proof>,
///                     (opaque type that contains data structures internal to Ursa.
///                      It should not be parsed and are likely to change in future versions).
///      "nonce": string
///    }
/// * `cred_req_metadata_json`: Credential request metadata json for further processing of received form Issuer credential.
///     Note: cred_req_metadata_json mustn't be shared with Issuer.
pub fn prover_create_credential_req(wallet_handle: WalletHandle, prover_did: &str, cred_offer_json: &str, cred_def_json: &str, master_secret_id: &str) -> Box<dyn Future<Item=(String, String), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

    let err = _prover_create_credential_req(command_handle, wallet_handle, prover_did, cred_offer_json, cred_def_json, master_secret_id, cb);

    ResultHandler::str_str(command_handle, err, receiver)
}

fn _prover_create_credential_req(command_handle: CommandHandle, wallet_handle: WalletHandle, prover_did: &str, cred_offer_json: &str, cred_def_json: &str, master_secret_id: &str, cb: Option<ResponseStringStringCB>) -> ErrorCode {
    let prover_did = c_str!(prover_did);
    let cred_offer_json = c_str!(cred_offer_json);
    let cred_def_json = c_str!(cred_def_json);
    let master_secret_id = c_str!(master_secret_id);

    ErrorCode::from(unsafe {
        anoncreds::indy_prover_create_credential_req(command_handle, wallet_handle, prover_did.as_ptr(), cred_offer_json.as_ptr(), cred_def_json.as_ptr(), master_secret_id.as_ptr(), cb)
    })
}

/// Set credential attribute tagging policy.
/// Writes a non-secret record marking attributes to tag, and optionally
/// updates tags on existing credentials on the credential definition to match.
///
/// The following tags are always present on write:
///     {
///         "schema_id": <credential schema id>,
///         "schema_issuer_did": <credential schema issuer did>,
///         "schema_name": <credential schema name>,
///         "schema_version": <credential schema version>,
///         "issuer_did": <credential issuer did>,
///         "cred_def_id": <credential definition id>,
///         "rev_reg_id": <credential revocation registry id>, // "None" as string if not present
///     }
///
/// The policy sets the following tags for each attribute it marks taggable, written to subsequent
/// credentials and (optionally) all existing credentials on the credential definition:
///     {
///         "attr::<attribute name>::marker": "1",
///         "attr::<attribute name>::value": <attribute raw value>,
///     }
///
/// # Arguments
/// command_handle: command handle to map callback to user context.
/// wallet_handle: wallet handle (created by Wallet::open_wallet).
/// cred_def_id: credential definition id
/// tag_attrs_json: JSON array with names of attributes to tag by policy, or null for all
/// retroactive: boolean, whether to apply policy to existing credentials on credential definition identifier
pub fn prover_set_credential_attr_tag_policy(wallet_handle: WalletHandle, cred_def_id: &str, tag_attrs_json: Option<&str>, retroactive: bool) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _prover_set_credential_attr_tag_policy(command_handle, wallet_handle, cred_def_id, tag_attrs_json, retroactive, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _prover_set_credential_attr_tag_policy(command_handle: CommandHandle, wallet_handle: WalletHandle, cred_def_id: &str, tag_attrs_json: Option<&str>, retroactive: bool, cb: Option<ResponseEmptyCB>) -> ErrorCode {
    let cred_def_id = c_str!(cred_def_id);
    let tag_attrs_json_str = opt_c_str!(tag_attrs_json);

    ErrorCode::from(unsafe {
        anoncreds::indy_prover_set_credential_attr_tag_policy(command_handle, wallet_handle, cred_def_id.as_ptr(), opt_c_ptr!(tag_attrs_json, tag_attrs_json_str), retroactive, cb)
    })
}

/// Get credential attribute tagging policy by credential definition id.
///
/// # Arguments
/// * `wallet_handle`: wallet handle (created by Wallet::open_wallet).
/// * `cred_id`: Identifier by which requested credential is stored in the wallet
///
/// # Returns
/// JSON array with all attributes that current policy marks taggable;
/// null for default policy (tag all credential attributes).
pub fn prover_get_credential_attr_tag_policy(wallet_handle: WalletHandle, cred_id: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _prover_get_credential_attr_tag_policy(command_handle, wallet_handle, cred_id, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _prover_get_credential_attr_tag_policy(command_handle: CommandHandle, wallet_handle: WalletHandle, cred_id: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let cred_id = c_str!(cred_id);

    ErrorCode::from(unsafe {
        anoncreds::indy_prover_get_credential_attr_tag_policy(command_handle, wallet_handle, cred_id.as_ptr(), cb)
    })
}

/// Check credential provided by Issuer for the given credential request,
/// updates the credential by a master secret and stores in a secure wallet.
///
/// To support efficient and flexible search the following tags will be created for stored credential:
///     {
///         "schema_id": <credential schema id>,
///         "schema_issuer_did": <credential schema issuer did>,
///         "schema_name": <credential schema name>,
///         "schema_version": <credential schema version>,
///         "issuer_did": <credential issuer did>,
///         "cred_def_id": <credential definition id>,
///         "rev_reg_id": <credential revocation registry id>, // "None" as string if not present
///         // for every attribute in <credential values> that credential attribute tagging policy marks taggable
///         "attr::<attribute name>::marker": "1",
///         "attr::<attribute name>::value": <attribute raw value>,
///     }
///
/// # Arguments
/// * `wallet_handle`: wallet handle (created by open_wallet).
/// * `cred_id`: (optional, default is a random one) identifier by which credential will be stored in the wallet
/// * `cred_req_metadata_json`: a credential request metadata created by create_credential_req
/// * `cred_json`: credential json received from issuer
/// * `cred_def_json`: credential definition json related to <cred_def_id> in <cred_json>
/// * `rev_reg_def_json`: revocation registry definition json related to <rev_reg_def_id> in <cred_json>
///
/// # Returns
/// * `out_cred_id` - identifier by which credential is stored in the wallet
pub fn prover_store_credential(wallet_handle: WalletHandle, cred_id: Option<&str>, cred_req_metadata_json: &str, cred_json: &str, cred_def_json: &str, rev_reg_def_json: Option<&str>) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _prover_store_credential(command_handle, wallet_handle, cred_id, cred_req_metadata_json, cred_json, cred_def_json, rev_reg_def_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _prover_store_credential(command_handle: CommandHandle, wallet_handle: WalletHandle, cred_id: Option<&str>, cred_req_metadata_json: &str, cred_json: &str, cred_def_json: &str, rev_reg_def_json: Option<&str>, cb: Option<ResponseStringCB>) -> ErrorCode {
    let cred_id_str = opt_c_str!(cred_id);
    let cred_req_metadata_json = c_str!(cred_req_metadata_json);
    let cred_json = c_str!(cred_json);
    let cred_def_json = c_str!(cred_def_json);
    let rev_reg_def_json_str = opt_c_str!(rev_reg_def_json);

    ErrorCode::from(unsafe {
        anoncreds::indy_prover_store_credential(command_handle, wallet_handle, opt_c_ptr!(cred_id, cred_id_str), cred_req_metadata_json.as_ptr(), cred_json.as_ptr(), cred_def_json.as_ptr(), opt_c_ptr!(rev_reg_def_json, rev_reg_def_json_str), cb)
    })
}

/// Gets human readable credentials according to the filter.
/// If filter is NULL, then all credentials are returned.
/// Credentials can be filtered by Issuer, credential_def and/or Schema.
///
/// # Arguments
/// * `wallet_handle`: wallet handle (created by open_wallet).
/// * `filter_json`: filter for credentials {
///    "schema_id": string, (Optional)
///    "schema_issuer_did": string, (Optional)
///    "schema_name": string, (Optional)
///    "schema_version": string, (Optional)
///    "issuer_did": string, (Optional)
///    "cred_def_id": string, (Optional)
///  }
///
/// # Returns
/// * `credentials_json` - [{
///     "referent": string, // cred_id in the wallet
///     "attrs": {"key1":"raw_value1", "key2":"raw_value2"},
///     "schema_id": string,
///     "cred_def_id": string,
///     "rev_reg_id": Optional<string>,
///     "cred_rev_id": Optional<string>
/// }]
pub fn prover_get_credentials(wallet_handle: WalletHandle, filter_json: Option<&str>) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _prover_get_credentials(command_handle, wallet_handle, filter_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _prover_get_credentials(command_handle: CommandHandle, wallet_handle: WalletHandle, filter_json: Option<&str>, cb: Option<ResponseStringCB>) -> ErrorCode {
    let filter_json_str = opt_c_str!(filter_json);

    ErrorCode::from(unsafe {
        anoncreds::indy_prover_get_credentials(command_handle, wallet_handle, opt_c_ptr!(filter_json, filter_json_str), cb)
    })
}

/// Search for credentials stored in wallet.
/// Credentials can be filtered by tags created during saving of credential.
///
/// Instead of immediately returning of fetched credentials
/// this call returns search_handle that can be used later
/// to fetch records by small batches (with fetch_credentials).
///
/// # Arguments
/// * `wallet_handle`: wallet handle (created by Wallet::open_wallet).
/// * `query_json`: Wql query filter for credentials searching based on tags.
///     where query: indy-sdk/doc/design/011-wallet-query-language/README.md
///
/// # Returns
/// * `search_handle`: Search handle that can be used later to fetch records by small batches (with fetch_credentials)
/// * `total_count`: Total count of records
pub fn prover_search_credentials(wallet_handle: WalletHandle, query_json: Option<&str>) -> Box<dyn Future<Item=(SearchHandle, usize), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_handle_usize();

    let err = _prover_search_credentials(command_handle, wallet_handle, query_json, cb);

    ResultHandler::handle_usize(command_handle, err, receiver)
}

fn _prover_search_credentials(command_handle: CommandHandle, wallet_handle: WalletHandle, query_json: Option<&str>, cb: Option<ResponseI32UsizeCB>) -> ErrorCode {
    let query_json_str = opt_c_str!(query_json);

    ErrorCode::from(unsafe {
        anoncreds::indy_prover_search_credentials(command_handle, wallet_handle, opt_c_ptr!(query_json, query_json_str), cb)
    })
}

/// Fetch next credentials for search.
///
/// # Arguments
/// * `search_handle`: Search handle (created by search_credentials)
/// * `count`: Count of credentials to fetch
///
/// # Returns
/// * `credentials_json`: List of human readable credentials:
///  [{
///     "referent": string, // cred_id in the wallet
///     "attrs": {"key1":"raw_value1", "key2":"raw_value2"},
///     "schema_id": string,
///     "cred_def_id": string,
///     "rev_reg_id": Optional<string>,
///     "cred_rev_id": Optional<string>
///  }]
pub fn prover_fetch_credentials(search_handle: SearchHandle, count: usize) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _prover_fetch_credentials(command_handle, search_handle, count, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _prover_fetch_credentials(command_handle: CommandHandle, search_handle: SearchHandle, count: usize, cb: Option<ResponseStringCB>) -> ErrorCode {
    ErrorCode::from(unsafe {
        anoncreds::indy_prover_fetch_credentials(command_handle, search_handle, count, cb)
    })
}

/// Close credentials search (make search handle invalid)
///
/// # Arguments
/// * `search_handle`: Search handle (created by search_credentials)
pub fn prover_close_credentials_search(search_handle: SearchHandle) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _prover_close_credentials_search(command_handle, search_handle, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _prover_close_credentials_search(command_handle: CommandHandle, search_handle: SearchHandle, cb: Option<ResponseEmptyCB>) -> ErrorCode {
    ErrorCode::from(unsafe {
        anoncreds::indy_prover_close_credentials_search(command_handle, search_handle, cb)
    })
}

/// Gets human readable credentials matching the given proof request.
///
/// NOTE: This method is deprecated because immediately returns all fetched credentials.
/// Use <search_credentials_for_proof_req> to fetch records by small batches.
///
/// # Arguments
/// * `wallet_handle`: wallet handle (created by Wallet::open_wallet).
/// * `proof_request_json`: proof request json
///     {
///         "name": string,
///         "version": string,
///         "nonce": string, - a de number represented as a string (use `generate_nonce` function to generate 80-bit number)
///         "requested_attributes": { // set of requested attributes
///              "<attr_referent>": <attr_info>, // see below
///              ...,
///         },
///         "requested_predicates": { // set of requested predicates
///              "<predicate_referent>": <predicate_info>, // see below
///              ...,
///          },
///         "non_revoked": Optional<<non_revoc_interval>>, // see below,
///                        // If specified prover must proof non-revocation
///                        // for date in this interval for each attribute
///                        // (can be overridden on attribute level)
///         "ver": Optional<str>  - proof request version:
///             - omit to use unqualified identifiers for restrictions
///             - "1.0" to use unqualified identifiers for restrictions
///             - "2.0" to use fully qualified identifiers for restrictions
///     }
///
/// where
/// `attr_referent`: Proof-request local identifier of requested attribute
/// `attr_info`: Describes requested attribute
///     {
///         "name": string, // attribute name, (case insensitive and ignore spaces)
///         "restrictions": Optional<filter_json>, // see above
///         "non_revoked": Optional<<non_revoc_interval>>, // see below,
///                        // If specified prover must proof non-revocation
///                        // for date in this interval this attribute
///                        // (overrides proof level interval)
///     }
/// `predicate_referent`: Proof-request local identifier of requested attribute predicate
/// `predicate_info`: Describes requested attribute predicate
///     {
///         "name": attribute name, (case insensitive and ignore spaces)
///         "p_type": predicate type (Currently ">=" only)
///         "p_value": int predicate value
///         "restrictions": Optional<filter_json>, // see above
///         "non_revoked": Optional<<non_revoc_interval>>, // see below,
///                        // If specified prover must proof non-revocation
///                        // for date in this interval this attribute
///                        // (overrides proof level interval)
///     }
/// `non_revoc_interval`: Defines non-revocation interval
///     {
///         "from": Optional<int>, // timestamp of interval beginning
///         "to": Optional<int>, // timestamp of interval ending
///     }
///
/// # Returns
/// * `credentials_json`: json with credentials for the given proof request.
///     {
///         "requested_attrs": {
///             "<attr_referent>": [{ cred_info: <credential_info>, interval: Optional<non_revoc_interval> }],
///             ...,
///         },
///         "requested_predicates": {
///             "requested_predicates": [{ cred_info: <credential_info>, timestamp: Optional<integer> }, { cred_info: <credential_2_info>, timestamp: Optional<integer> }],
///             "requested_predicate_2_referent": [{ cred_info: <credential_2_info>, timestamp: Optional<integer> }]
///         }
///     }, where credential is
///     {
///         "referent": <string>,
///         "attrs": {"attr_name" : "attr_raw_value"},
///         "schema_id": string,
///         "cred_def_id": string,
///         "rev_reg_id": Optional<int>,
///         "cred_rev_id": Optional<int>,
///     }
pub fn prover_get_credentials_for_proof_req(wallet_handle: WalletHandle, proof_request_json: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _prover_get_credentials_for_proof_req(command_handle, wallet_handle, proof_request_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _prover_get_credentials_for_proof_req(command_handle: CommandHandle, wallet_handle: WalletHandle, proof_request_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let proof_request_json = c_str!(proof_request_json);

    ErrorCode::from(unsafe {
        anoncreds::indy_prover_get_credentials_for_proof_req(command_handle, wallet_handle, proof_request_json.as_ptr(), cb)
    })
}

/// Search for credentials matching the given proof request.
///
/// Instead of immediately returning of fetched credentials
/// this call returns search_handle that can be used later
/// to fetch records by small batches (with fetch_credentials_for_proof_req).
///
/// # Arguments
/// * `wallet_handle`: wallet handle (created by Wallet::open_wallet).
/// * `proof_request_json`: proof request json
///     {
///         "name": string,
///         "version": string,
///         "nonce": string, - a decimal number represented as a string (use `generate_nonce` function to generate 80-bit number)
///         "requested_attributes": { // set of requested attributes
///              "<attr_referent>": <attr_info>, // see below
///              ...,
///         },
///         "requested_predicates": { // set of requested predicates
///              "<predicate_referent>": <predicate_info>, // see below
///              ...,
///          },
///         "non_revoked": Optional<<non_revoc_interval>>, // see below,
///                        // If specified prover must proof non-revocation
///                        // for date in this interval for each attribute
///                        // (can be overridden on attribute level)
///         "ver": Optional<str>  - proof request version:
///             - omit to use unqualified identifiers for restrictions
///             - "1.0" to use unqualified identifiers for restrictions
///             - "2.0" to use fully qualified identifiers for restrictions
///     }
/// * `extra_query_json`: (Optional) List of extra queries that will be applied to correspondent attribute/predicate:
///     {
///         "<attr_referent>": <wql query>,
///         "<predicate_referent>": <wql query>,
///     }
/// where wql query: indy-sdk/doc/design/011-wallet-query-language/README.md
///
/// where
/// `attr_referent`: Proof-request local identifier of requested attribute
/// `attr_info`: Describes requested attribute
///     {
///         "name": string, // attribute name, (case insensitive and ignore spaces)
///         "restrictions": Optional<filter_json>, // see above
///         "non_revoked": Optional<<non_revoc_interval>>, // see below,
///                        // If specified prover must proof non-revocation
///                        // for date in this interval this attribute
///                        // (overrides proof level interval)
///     }
/// `predicate_referent`: Proof-request local identifier of requested attribute predicate
/// `predicate_info`: Describes requested attribute predicate
///     {
///         "name": attribute name, (case insensitive and ignore spaces)
///         "p_type": predicate type (Currently ">=" only)
///         "p_value": int predicate value
///         "restrictions": Optional<filter_json>, // see above
///         "non_revoked": Optional<<non_revoc_interval>>, // see below,
///                        // If specified prover must proof non-revocation
///                        // for date in this interval this attribute
///                        // (overrides proof level interval)
///     }
/// `non_revoc_interval`: Defines non-revocation interval
///     {
///         "from": Optional<int>, // timestamp of interval beginning
///         "to": Optional<int>, // timestamp of interval ending
///     }
///
/// # Returns
/// * `search_handle`: Search handle that can be used later to fetch records by small batches (with fetch_credentials_for_proof_req)
pub fn prover_search_credentials_for_proof_req(wallet_handle: WalletHandle,
                                               proof_request_json: &str,
                                               extra_query_json: Option<&str>) -> Box<dyn Future<Item=CommandHandle, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_handle();

    let err = _prover_search_credentials_for_proof_req(command_handle, wallet_handle, proof_request_json, extra_query_json, cb);

    ResultHandler::handle(command_handle, err, receiver)
}

fn _prover_search_credentials_for_proof_req(command_handle: CommandHandle,
                                            wallet_handle: WalletHandle,
                                            proof_request_json: &str,
                                            extra_query_json: Option<&str>, cb: Option<ResponseI32CB>) -> ErrorCode {
    let proof_request_json = c_str!(proof_request_json);
    let extra_query_json_str = opt_c_str!(extra_query_json);

    ErrorCode::from(unsafe {
        anoncreds::indy_prover_search_credentials_for_proof_req(command_handle, wallet_handle, proof_request_json.as_ptr(), opt_c_ptr!(extra_query_json, extra_query_json_str), cb)
    })
}

/// Fetch next credentials for the requested item using proof request search
/// handle (created by search_credentials_for_proof_req).
///
/// # Arguments
/// * `search_handle`: Search handle (created by search_credentials_for_proof_req)
/// * `item_referent`: Referent of attribute/predicate in the proof request
/// * `count`: Count of credentials to fetch
///
/// # Returns
/// * `credentials_json`: List of credentials for the given proof request.
///     [{
///         cred_info: <credential_info>,
///         interval: Optional<non_revoc_interval>
///     }]
/// where
/// `credential_info`:
///     {
///         "referent": <string>,
///         "attrs": {"attr_name" : "attr_raw_value"},
///         "schema_id": string,
///         "cred_def_id": string,
///         "rev_reg_id": Optional<int>,
///         "cred_rev_id": Optional<int>,
///     }
/// `non_revoc_interval`:
///     {
///         "from": Optional<int>, // timestamp of interval beginning
///         "to": Optional<int>, // timestamp of interval ending
///     }
/// NOTE: The list of length less than the requested count means that search iterator
/// correspondent to the requested <item_referent> is completed.
pub fn prover_fetch_credentials_for_proof_req(search_handle: SearchHandle, item_referent: &str, count: usize) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _prover_fetch_credentials_for_proof_req(command_handle, search_handle, item_referent, count, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _prover_fetch_credentials_for_proof_req(command_handle: CommandHandle, search_handle: SearchHandle, item_referent: &str, count: usize, cb: Option<ResponseStringCB>) -> ErrorCode {
    let item_referent = c_str!(item_referent);

    ErrorCode::from(unsafe {
        anoncreds::indy_prover_fetch_credentials_for_proof_req(command_handle, search_handle, item_referent.as_ptr(), count, cb)
    })
}

/// Close credentials search for proof request (make search handle invalid)
///
/// # Arguments
/// * `search_handle`: Search handle (created by search_credentials_for_proof_req)
pub fn prover_close_credentials_search_for_proof_req(search_handle: SearchHandle) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _prover_close_credentials_search_for_proof_req(command_handle, search_handle, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _prover_close_credentials_search_for_proof_req(command_handle: CommandHandle, search_handle: SearchHandle, cb: Option<ResponseEmptyCB>) -> ErrorCode {
    ErrorCode::from(unsafe {
        anoncreds::indy_prover_close_credentials_search_for_proof_req(command_handle, search_handle, cb)
    })
}

/// Creates a proof according to the given proof request
/// Either a corresponding credential with optionally revealed attributes or self-attested attribute must be provided
/// for each requested attribute (see indy_prover_get_credentials_for_pool_req).
/// A proof request may request multiple credentials from different schemas and different issuers.
/// All required schemas, public keys and revocation registries must be provided.
/// The proof request also contains nonce.
/// The proof contains either proof or self-attested attribute value for each requested attribute.
///
/// # Arguments
/// * `wallet_handle`: wallet handle (created by Wallet::open_wallet).
/// * `proof_request_json`: proof request json
///     {
///         "name": string,
///         "version": string,
///         "nonce": string, - a decimal number represented as a string (use `generate_nonce` function to generate 80-bit number)
///         "requested_attributes": { // set of requested attributes
///              "<attr_referent>": <attr_info>, // see below
///              ...,
///         },
///         "requested_predicates": { // set of requested predicates
///              "<predicate_referent>": <predicate_info>, // see below
///              ...,
///          },
///         "non_revoked": Optional<<non_revoc_interval>>, // see below,
///                        // If specified prover must proof non-revocation
///                        // for date in this interval for each attribute
///                        // (can be overridden on attribute level)
///         "ver": Optional<str>  - proof request version:
///             - omit to use unqualified identifiers for restrictions
///             - "1.0" to use unqualified identifiers for restrictions
///             - "2.0" to use fully qualified identifiers for restrictions
///     }
/// * `requested_credentials_json`: either a credential or self-attested attribute for each requested attribute
///     {
///         "self_attested_attributes": {
///             "self_attested_attribute_referent": string
///         },
///         "requested_attributes": {
///             "requested_attribute_referent_1": {"cred_id": string, "timestamp": Optional<number>, revealed: <bool> }},
///             "requested_attribute_referent_2": {"cred_id": string, "timestamp": Optional<number>, revealed: <bool> }}
///         },
///         "requested_predicates": {
///             "requested_predicates_referent_1": {"cred_id": string, "timestamp": Optional<number> }},
///         }
///     }
/// * `master_secret_id`: the id of the master secret stored in the wallet
/// * `schemas_json`: all schemas json participating in the proof request
///     {
///         <schema1_id>: <schema1_json>,
///         <schema2_id>: <schema2_json>,
///         <schema3_id>: <schema3_json>,
///     }
/// * `credential_defs_json`: all credential definitions json participating in the proof request
///     {
///         "cred_def1_id": <credential_def1_json>,
///         "cred_def2_id": <credential_def2_json>,
///         "cred_def3_id": <credential_def3_json>,
///     }
/// * `rev_states_json`: all revocation states json participating in the proof request
///     {
///         "rev_reg_def1_id or credential_1_id": {
///             "timestamp1": <rev_state1>,
///             "timestamp2": <rev_state2>,
///         },
///         "rev_reg_def2_id or credential_2_id": {
///             "timestamp3": <rev_state3>
///         },
///         "rev_reg_def3_id or credential_3_id": {
///             "timestamp4": <rev_state4>
///         },
///     } - Note: use credential_id instead rev_reg_id in case proving several credentials from the same revocation registry.
///
/// where
/// where wql query: indy-sdk/doc/design/011-wallet-query-language/README.md
/// attr_referent: Proof-request local identifier of requested attribute
/// attr_info: Describes requested attribute
///     {
///         "name": string, // attribute name, (case insensitive and ignore spaces)
///         "restrictions": Optional<wql query>,
///         "non_revoked": Optional<<non_revoc_interval>>, // see below,
///                        // If specified prover must proof non-revocation
///                        // for date in this interval this attribute
///                        // (overrides proof level interval)
///     }
/// predicate_referent: Proof-request local identifier of requested attribute predicate
/// predicate_info: Describes requested attribute predicate
///     {
///         "name": attribute name, (case insensitive and ignore spaces)
///         "p_type": predicate type (Currently >= only)
///         "p_value": predicate value
///         "restrictions": Optional<wql query>,
///         "non_revoked": Optional<<non_revoc_interval>>, // see below,
///                        // If specified prover must proof non-revocation
///                        // for date in this interval this attribute
///                        // (overrides proof level interval)
///     }
/// non_revoc_interval: Defines non-revocation interval
///     {
///         "from": Optional<int>, // timestamp of interval beginning
///         "to": Optional<int>, // timestamp of interval ending
///     }
///
/// # Returns
/// * `proof_json` - proof json
/// For each requested attribute either a proof (with optionally revealed attribute value) or
/// self-attested attribute value is provided.
/// Each proof is associated with a credential and corresponding schema_id, cred_def_id, rev_reg_id and timestamp.
/// There is also aggregated proof part common for all credential proofs.
///     {
///         "requested_proof": {
///             "revealed_attrs": {
///                 "requested_attr1_id": {sub_proof_index: number, raw: string, encoded: string},
///                 "requested_attr4_id": {sub_proof_index: number: string, encoded: string},
///             },
///             "unrevealed_attrs": {
///                 "requested_attr3_id": {sub_proof_index: number}
///             },
///             "self_attested_attrs": {
///                 "requested_attr2_id": self_attested_value,
///             },
///             "requested_predicates": {
///                 "requested_predicate_1_referent": {sub_proof_index: int},
///                 "requested_predicate_2_referent": {sub_proof_index: int},
///             }
///         }
///         "proof": {
///             "proofs": [ <credential_proof>, <credential_proof>, <credential_proof> ],
///             "aggregated_proof": <aggregated_proof>
///         } (opaque type that contains data structures internal to Ursa.
///           It should not be parsed and are likely to change in future versions).
///         "identifiers": [{schema_id, cred_def_id, Optional<rev_reg_id>, Optional<timestamp>}]
///     }
pub fn prover_create_proof(wallet_handle: WalletHandle, proof_req_json: &str, requested_credentials_json: &str, master_secret_id: &str, schemas_json: &str, credential_defs_json: &str, rev_states_json: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _prover_create_proof(command_handle, wallet_handle, proof_req_json, requested_credentials_json, master_secret_id, schemas_json, credential_defs_json, rev_states_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _prover_create_proof(command_handle: CommandHandle, wallet_handle: WalletHandle, proof_req_json: &str, requested_credentials_json: &str, master_secret_id: &str, schemas_json: &str, credential_defs_json: &str, rev_states_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let proof_req_json = c_str!(proof_req_json);
    let requested_credentials_json = c_str!(requested_credentials_json);
    let master_secret_id = c_str!(master_secret_id);
    let schemas_json = c_str!(schemas_json);
    let credential_defs_json = c_str!(credential_defs_json);
    let rev_states_json = c_str!(rev_states_json);

    ErrorCode::from(unsafe {
        anoncreds::indy_prover_create_proof(command_handle, wallet_handle, proof_req_json.as_ptr(), requested_credentials_json.as_ptr(), master_secret_id.as_ptr(), schemas_json.as_ptr(), credential_defs_json.as_ptr(), rev_states_json.as_ptr(), cb)
    })
}


/// Verifies a proof (of multiple credential).
/// All required schemas, public keys and revocation registries must be provided.
///
/// IMPORTANT: You must use *_id's (`schema_id`, `cred_def_id`, `rev_reg_id`) listed in `proof[identifiers]`
/// as the keys for corresponding `schemas_json`, `credential_defs_json`, `rev_reg_defs_json`, `rev_regs_json` objects.
///
/// # Arguments
/// * `wallet_handle`: wallet handle (created by Wallet::open_wallet).
/// * `proof_request_json`: proof request json
///     {
///         "name": string,
///         "version": string,
///         "nonce": string, - a decimal number represented as a string (use `generate_nonce` function to generate 80-bit number)
///         "requested_attributes": { // set of requested attributes
///              "<attr_referent>": <attr_info>, // see below
///              ...,
///         },
///         "requested_predicates": { // set of requested predicates
///              "<predicate_referent>": <predicate_info>, // see below
///              ...,
///          },
///         "non_revoked": Optional<<non_revoc_interval>>, // see below,
///                        // If specified prover must proof non-revocation
///                        // for date in this interval for each attribute
///                        // (can be overridden on attribute level)
///         "ver": Optional<str>  - proof request version:
///             - omit to use unqualified identifiers for restrictions
///             - "1.0" to use unqualified identifiers for restrictions
///             - "2.0" to use fully qualified identifiers for restrictions
///     }
/// * `proof_json`: created for request proof json
///     {
///         "requested_proof": {
///             "revealed_attrs": {
///                 "requested_attr1_id": {sub_proof_index: number, raw: string, encoded: string},
///                 "requested_attr4_id": {sub_proof_index: number: string, encoded: string},
///             },
///             "unrevealed_attrs": {
///                 "requested_attr3_id": {sub_proof_index: number}
///             },
///             "self_attested_attrs": {
///                 "requested_attr2_id": self_attested_value,
///             },
///             "requested_predicates": {
///                 "requested_predicate_1_referent": {sub_proof_index: int},
///                 "requested_predicate_2_referent": {sub_proof_index: int},
///             }
///         }
///         "proof": {
///             "proofs": [ <credential_proof>, <credential_proof>, <credential_proof> ],
///             "aggregated_proof": <aggregated_proof>
///         }
///         "identifiers": [{schema_id, cred_def_id, Optional<rev_reg_id>, Optional<timestamp>}]
///     }
/// * `schemas_json`: all schema jsons participating in the proof
///     {
///         <schema1_id>: <schema1_json>,
///         <schema2_id>: <schema2_json>,
///         <schema3_id>: <schema3_json>,
///     }
/// * `credential_defs_json`: all credential definitions json participating in the proof
///     {
///         "cred_def1_id": <credential_def1_json>,
///         "cred_def2_id": <credential_def2_json>,
///         "cred_def3_id": <credential_def3_json>,
///     }
/// * `rev_reg_defs_json`: all revocation registry definitions json participating in the proof
///     {
///         "rev_reg_def1_id": <rev_reg_def1_json>,
///         "rev_reg_def2_id": <rev_reg_def2_json>,
///         "rev_reg_def3_id": <rev_reg_def3_json>,
///     }
/// * `rev_regs_json`: all revocation registries json participating in the proof
///     {
///         "rev_reg_def1_id": {
///             "timestamp1": <rev_reg1>,
///             "timestamp2": <rev_reg2>,
///         },
///         "rev_reg_def2_id": {
///             "timestamp3": <rev_reg3>
///         },
///         "rev_reg_def3_id": {
///             "timestamp4": <rev_reg4>
///         },
///     }
///
/// # Returns
/// * `valid`: true - if signature is valid, false - otherwise
pub fn verifier_verify_proof(proof_request_json: &str, proof_json: &str, schemas_json: &str, credential_defs_json: &str, rev_reg_defs_json: &str, rev_regs_json: &str) -> Box<dyn Future<Item=bool, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_bool();

    let err = _verifier_verify_proof(command_handle, proof_request_json, proof_json, schemas_json, credential_defs_json, rev_reg_defs_json, rev_regs_json, cb);

    ResultHandler::bool(command_handle, err, receiver)
}

fn _verifier_verify_proof(command_handle: CommandHandle, proof_request_json: &str, proof_json: &str, schemas_json: &str, credential_defs_json: &str, rev_reg_defs_json: &str, rev_regs_json: &str, cb: Option<ResponseBoolCB>) -> ErrorCode {
    let proof_request_json = c_str!(proof_request_json);
    let proof_json = c_str!(proof_json);
    let schemas_json = c_str!(schemas_json);
    let credential_defs_json = c_str!(credential_defs_json);
    let rev_reg_defs_json = c_str!(rev_reg_defs_json);
    let rev_regs_json = c_str!(rev_regs_json);

    ErrorCode::from(unsafe {
        anoncreds::indy_verifier_verify_proof(command_handle, proof_request_json.as_ptr(), proof_json.as_ptr(), schemas_json.as_ptr(), credential_defs_json.as_ptr(), rev_reg_defs_json.as_ptr(), rev_regs_json.as_ptr(), cb)
    })
}


/// Create revocation state for a credential that corresponds to a particular time.
///
/// Note that revocation delta must cover the whole registry existence time.
/// You can use `from`: `0` and `to`: `needed_time` as parameters for building request to get correct revocation delta.
///
/// The resulting revocation state and provided timestamp can be saved and reused later with applying a new
/// revocation delta with `update_revocation_state` function.
/// This new delta should be received with parameters: `from`: `timestamp` and `to`: `needed_time`.
///
/// # Arguments
/// * `blob_storage_reader_handle`: configuration of blob storage reader handle that will allow to read revocation tails
/// * `rev_reg_def_json`: revocation registry definition json
/// * `rev_reg_delta_json`: revocation registry delta which covers the whole registry existence time
/// * `timestamp`: time represented as a total number of seconds from Unix Epoch
/// * `cred_rev_id`: user credential revocation id in revocation registry
///
/// # Returns
/// * `revocation_state_json`:
/// {
///     "rev_reg": <revocation registry>,
///     "witness": <witness>,  (opaque type that contains data structures internal to Ursa.
///                             It should not be parsed and are likely to change in future versions).
///     "timestamp" : integer
/// }
pub fn create_revocation_state(blob_storage_reader_handle: BlobStorageReaderHandle, rev_reg_def_json: &str, rev_reg_delta_json: &str, timestamp: u64, cred_rev_id: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _create_revocation_state(command_handle, blob_storage_reader_handle, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _create_revocation_state(command_handle: CommandHandle, blob_storage_reader_handle: BlobStorageReaderHandle, rev_reg_def_json: &str, rev_reg_delta_json: &str, timestamp: u64, cred_rev_id: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let rev_reg_def_json = c_str!(rev_reg_def_json);
    let rev_reg_delta_json = c_str!(rev_reg_delta_json);
    let cred_rev_id = c_str!(cred_rev_id);

    ErrorCode::from(unsafe {
        anoncreds::indy_create_revocation_state(command_handle, blob_storage_reader_handle, rev_reg_def_json.as_ptr(), rev_reg_delta_json.as_ptr(), timestamp, cred_rev_id.as_ptr(), cb)
    })
}

/// Create a new revocation state for a credential based on a revocation state created before.
/// Note that provided revocation delta must cover the registry gap from based state creation until the specified time
/// (this new delta should be received with parameters: `from`: `state_timestamp` and `to`: `needed_time`).
///
/// This function reduces the calculation time.
///
/// The resulting revocation state and provided timestamp can be saved and reused later by applying a new revocation delta again.
///
/// # Arguments
/// * `blob_storage_reader_handle`: configuration of blob storage reader handle that will allow to read revocation tails
/// * `rev_state_json`: revocation registry state json
/// * `rev_reg_def_json`: revocation registry definition json
/// * `rev_reg_delta_json`: revocation registry definition delta which covers the gap form original `rev_state_json` creation till the requested timestamp.
/// * `timestamp`: time represented as a total number of seconds from Unix Epoch
/// * `cred_rev_id`: user credential revocation id in revocation registry
///
/// # Returns
/// * `revocation_state_json`:
/// {
///     "rev_reg": <revocation registry>,
///     "witness": <witness>,  (opaque type that contains data structures internal to Ursa.
///                            It should not be parsed and are likely to change in future versions).
///     "timestamp" : integer
/// }
pub fn update_revocation_state(blob_storage_reader_handle: BlobStorageReaderHandle, rev_state_json: &str, rev_reg_def_json: &str, rev_reg_delta_json: &str, timestamp: u64, cred_rev_id: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _update_revocation_state(command_handle, blob_storage_reader_handle, rev_state_json, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _update_revocation_state(command_handle: CommandHandle, blob_storage_reader_handle: BlobStorageReaderHandle, rev_state_json: &str, rev_reg_def_json: &str, rev_reg_delta_json: &str, timestamp: u64, cred_rev_id: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let rev_state_json = c_str!(rev_state_json);
    let rev_reg_def_json = c_str!(rev_reg_def_json);
    let rev_reg_delta_json = c_str!(rev_reg_delta_json);
    let cred_rev_id = c_str!(cred_rev_id);

    ErrorCode::from(unsafe {
        anoncreds::indy_update_revocation_state(command_handle, blob_storage_reader_handle, rev_state_json.as_ptr(), rev_reg_def_json.as_ptr(), rev_reg_delta_json.as_ptr(), timestamp, cred_rev_id.as_ptr(), cb)
    })
}

/// Generates 80-bit numbers that can be used as a nonce for proof request.
///
/// # Arguments
/// * `blob_storage_reader_handle`: configuration of blob storage reader handle that will allow to read revocation tails
/// * `rev_state_json`: revocation registry state json
/// * `rev_reg_def_json`: revocation registry definition json
/// * `rev_reg_delta_json`: revocation registry definition delta json
/// * `timestamp`: time represented as a total number of seconds from Unix Epoch
/// * `cred_rev_id`: user credential revocation id in revocation registry
///
/// # Returns
/// * `nonce`: generated number as a string
pub fn generate_nonce() -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _generate_nonce(command_handle, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _generate_nonce(command_handle: CommandHandle, cb: Option<ResponseStringCB>) -> ErrorCode {
    ErrorCode::from(unsafe {
        anoncreds::indy_generate_nonce(command_handle, cb)
    })
}

/// Get unqualified form (short form without method) of a fully qualified entity like DID.
///
/// This function should be used to the proper casting of fully qualified entity to unqualified form in the following cases:
///     Issuer, which works with fully qualified identifiers, creates a Credential Offer for Prover, which doesn't support fully qualified identifiers.
///     Verifier prepares a Proof Request based on fully qualified identifiers or Prover, which doesn't support fully qualified identifiers.
///     another case when casting to unqualified form needed
///
/// # Arguments
/// * `entity`: target entity to disqualify. Can be one of:
///             Did
///             SchemaId
///             CredentialDefinitionId
///             RevocationRegistryId
///             Schema
///             CredentialDefinition
///             RevocationRegistryDefinition
///             CredentialOffer
///             CredentialRequest
///             ProofRequest
///
/// # Returns
/// * `res`: entity either in unqualified form or original if casting isn't possible
pub fn to_unqualified(entity: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _to_unqualified(command_handle, entity, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _to_unqualified(command_handle: CommandHandle, entity: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let entity = c_str!(entity);

    ErrorCode::from(unsafe {
        anoncreds::indy_to_unqualified(command_handle, entity.as_ptr(), cb)
    })
}
