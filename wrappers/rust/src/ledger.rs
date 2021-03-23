use serde_json::json;
use {ErrorCode, IndyError};

use std::ffi::CString;
use std::ptr::null;

use futures::Future;

use ffi::ledger;
use ffi::{ResponseStringCB,
          ResponseStringStringCB,
          ResponseStringStringU64CB};

use utils::callbacks::{ClosureHandler, ResultHandler};
use {WalletHandle, CommandHandle, PoolHandle};

/// Signs and submits request message to validator pool.
///
/// Adds submitter information to passed request json, signs it with submitter
/// sign key (see Crypto::sign), and sends signed request message
/// to validator pool (see Pool::write_request).
///
/// # Arguments
/// * `pool_handle` - pool handle (created by Pool::open_ledger).
/// * `wallet_handle` - wallet handle (created by Wallet::open).
/// * `submitter_did` - Id of Identity stored in secured Wallet.
/// * `request_json` - Request data json.
///
/// # Returns
/// Request result as json.
pub fn sign_and_submit_request(pool_handle: PoolHandle, wallet_handle: WalletHandle, submitter_did: &str, request_json: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _sign_and_submit_request(command_handle, pool_handle, wallet_handle, submitter_did, request_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _sign_and_submit_request(command_handle: CommandHandle, pool_handle: PoolHandle, wallet_handle: WalletHandle, submitter_did: &str, request_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did = c_str!(submitter_did);
    let request_json = c_str!(request_json);

    ErrorCode::from(unsafe {
        ledger::indy_sign_and_submit_request(command_handle,
                                             pool_handle,
                                             wallet_handle,
                                             submitter_did.as_ptr(),
                                             request_json.as_ptr(),
                                             cb)
    })
}

/// Publishes request message to validator pool (no signing, unlike sign_and_submit_request).
///
/// The request is sent to the validator pool as is. It's assumed that it's already prepared.
///
/// # Arguments
/// * `pool_handle` - pool handle (created by Pool::open_ledger).
/// * `request_json` - Request data json.
///
/// # Returns
/// Request result as json.
pub fn submit_request(pool_handle: PoolHandle, request_json: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _submit_request(command_handle, pool_handle, request_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _submit_request(command_handle: CommandHandle, pool_handle: PoolHandle, request_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let request_json = c_str!(request_json);

    ErrorCode::from(unsafe { ledger::indy_submit_request(command_handle, pool_handle, request_json.as_ptr(), cb) })
}

pub fn submit_action(pool_handle: PoolHandle, request_json: &str, nodes: Option<&str>, wait_timeout: Option<i32>) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _submit_action(command_handle, pool_handle, request_json, nodes, wait_timeout, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _submit_action(command_handle: CommandHandle, pool_handle: PoolHandle, request_json: &str, nodes: Option<&str>, wait_timeout: Option<i32>, cb: Option<ResponseStringCB>) -> ErrorCode {
    let request_json = c_str!(request_json);
    let nodes_str = opt_c_str!(nodes);

    ErrorCode::from(unsafe {
        ledger::indy_submit_action(command_handle, pool_handle, request_json.as_ptr(), opt_c_ptr!(nodes, nodes_str), wait_timeout.unwrap_or(-1), cb)
    })
}

/// Signs request message.
///
/// Adds submitter information to passed request json, signs it with submitter
/// sign key (see Crypto::sign).
///
/// # Arguments
/// * `wallet_handle` - wallet handle (created by Wallet::open).
/// * `submitter_did` - Id of Identity stored in secured Wallet.
/// * `request_json` - Request data json.
///
/// # Returns
/// Signed request json.
pub fn sign_request(wallet_handle: WalletHandle, submitter_did: &str, request_json: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _sign_request(command_handle, wallet_handle, submitter_did, request_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _sign_request(command_handle: CommandHandle, wallet_handle: WalletHandle, submitter_did: &str, request_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did = c_str!(submitter_did);
    let request_json = c_str!(request_json);

    ErrorCode::from(unsafe { ledger::indy_sign_request(command_handle, wallet_handle, submitter_did.as_ptr(), request_json.as_ptr(), cb) })
}

/// Multi signs request message.
///
/// Adds submitter information to passed request json, signs it with submitter
/// sign key (see Crypto::sign).
///
/// # Arguments
/// * `wallet_handle` - wallet handle (created by Wallet::open).
/// * `submitter_did` - Id of Identity stored in secured Wallet.
/// * `request_json` - Request data json.
///
/// # Returns
/// Signed request json.
pub fn multi_sign_request(wallet_handle: WalletHandle, submitter_did: &str, request_json: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _multi_sign_request(command_handle, wallet_handle, submitter_did, request_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _multi_sign_request(command_handle: CommandHandle, wallet_handle: WalletHandle, submitter_did: &str, request_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did = c_str!(submitter_did);
    let request_json = c_str!(request_json);

    ErrorCode::from(unsafe { ledger::indy_multi_sign_request(command_handle, wallet_handle, submitter_did.as_ptr(), request_json.as_ptr(), cb) })
}

/// Builds a request to get a DDO.
///
/// # Arguments
/// * `submitter_did` - Id of Identity stored in secured Wallet
/// * `target_did` - Id of Identity stored in secured Wallet.
///
/// # Returns
/// Request result as json.
pub fn build_get_ddo_request(submitter_did: Option<&str>, target_did: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_get_ddo_request(command_handle, submitter_did, target_did, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_get_ddo_request(command_handle: CommandHandle, submitter_did: Option<&str>, target_did: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did_str = opt_c_str!(submitter_did);
    let target_did = c_str!(target_did);

    ErrorCode::from(unsafe { ledger::indy_build_get_ddo_request(command_handle, opt_c_ptr!(submitter_did, submitter_did_str), target_did.as_ptr(), cb) })
}

/// Builds a NYM request. Request to create a new NYM record for a specific user.
///
/// # Arguments
/// * `submitter_did` - Identifier (DID) of the transaction author as base58-encoded string.
///                Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
/// * `target_did` - Target DID as base58-encoded string for 16 or 32 bit DID value.
/// * `verkey` - Target identity verification key as base58-encoded string.
/// * `data`
/// * `role` - Role of a user NYM record:
///                             null (common USER)
///                             TRUSTEE
///                             STEWARD
///                             TRUST_ANCHOR
///                             ENDORSER - equal to TRUST_ANCHOR that will be removed soon
///                             NETWORK_MONITOR
///                             empty string to reset role
///
/// # Returns
/// Request result as json.
pub fn build_nym_request(submitter_did: &str, target_did: &str, verkey: Option<&str>, data: Option<&str>, role: Option<&str>) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_nym_request(command_handle, submitter_did, target_did, verkey, data, role, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_nym_request(command_handle: CommandHandle,
                      submitter_did: &str,
                      target_did: &str,
                      verkey: Option<&str>,
                      data: Option<&str>,
                      role: Option<&str>,
                      cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did = c_str!(submitter_did);
    let target_did = c_str!(target_did);

    let verkey_str = opt_c_str!(verkey);
    let data_str = opt_c_str!(data);
    let role_str = opt_c_str!(role);

    ErrorCode::from(unsafe {
        ledger::indy_build_nym_request(command_handle,
                                       submitter_did.as_ptr(),
                                       target_did.as_ptr(),
                                       opt_c_ptr!(verkey, verkey_str),
                                       opt_c_ptr!(data, data_str),
                                       opt_c_ptr!(role, role_str),
                                       cb)
    })
}

/// Builds a GET_NYM request. Request to get information about a DID (NYM).
///
/// # Arguments
/// * `submitter_did` - (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
/// * `target_did` - Target DID as base58-encoded string for 16 or 32 bit DID value.
///
/// # Returns
/// Request result as json.
pub fn build_get_nym_request(submitter_did: Option<&str>, target_did: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_get_nym_request(command_handle, submitter_did, target_did, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_get_nym_request(command_handle: CommandHandle, submitter_did: Option<&str>, target_did: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did_str = opt_c_str!(submitter_did);
    let target_did = c_str!(target_did);

    ErrorCode::from(unsafe { ledger::indy_build_get_nym_request(command_handle, opt_c_ptr!(submitter_did, submitter_did_str), target_did.as_ptr(), cb) })
}

/// Parse a GET_NYM response to get NYM data.
///
/// # Arguments
/// * `get_nym_response`: response on GET_NYM request.
///
/// # Returns
/// NYM data
/// {
///     did: DID as base58-encoded string for 16 or 32 bit DID value.
///     verkey: verification key as base58-encoded string.
///     role: Role associated number
///                             null (common USER)
///                             0 - TRUSTEE
///                             2 - STEWARD
///                             101 - TRUST_ANCHOR
///                             101 - ENDORSER - equal to TRUST_ANCHOR that will be removed soon
///                             201 - NETWORK_MONITOR
/// }
pub fn parse_get_nym_response(get_nym_response: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _parse_get_nym_response(command_handle, get_nym_response, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _parse_get_nym_response(command_handle: CommandHandle, get_nym_response: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let get_nym_response = c_str!(get_nym_response);

    ErrorCode::from(unsafe { ledger::indy_parse_get_nym_response(command_handle, get_nym_response.as_ptr(), cb) })
}

/// Builds a GET_TXN request. Request to get any transaction by its seq_no.
///
/// # Arguments
/// `submitter_did` - (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
/// * `ledger_type` - (Optional) type of the ledger the requested transaction belongs to:
///     DOMAIN - used default,
///     POOL,
///     CONFIG
/// * `seq_no` - seq_no of transaction in ledger.
///
/// # Returns
/// Request result as json.
pub fn build_get_txn_request(submitter_did: Option<&str>, ledger_type: Option<&str>, seq_no: i32) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_get_txn_request(command_handle, submitter_did, ledger_type, seq_no, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_get_txn_request(command_handle: CommandHandle, submitter_did: Option<&str>, ledger_type: Option<&str>, seq_no: i32, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did_str = opt_c_str!(submitter_did);
    let ledger_type_str = opt_c_str!(ledger_type);

    ErrorCode::from(unsafe { ledger::indy_build_get_txn_request(command_handle, opt_c_ptr!(submitter_did, submitter_did_str), opt_c_ptr!(ledger_type, ledger_type_str), seq_no, cb) })
}

/// Builds an ATTRIB request. Request to add attribute to a NYM record.
///
/// # Arguments
/// * `submitter_did` - Identifier (DID) of the transaction author as base58-encoded string.
///                Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
/// * `target_did` - Target DID as base58-encoded string for 16 or 32 bit DID value.
/// * `hash` - (Optional) Hash of attribute data.
/// * `raw` - (Optional) Json, where key is attribute name and value is attribute value.
/// * `enc` - (Optional) Encrypted value attribute data.
///
/// # Returns
/// Request result as json.
pub fn build_attrib_request(submitter_did: &str, target_did: &str, hash: Option<&str>, raw: Option<&str>, enc: Option<&str>) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_attrib_request(command_handle, submitter_did, target_did, hash, raw, enc, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_attrib_request(command_handle: CommandHandle, submitter_did: &str, target_did: &str, hash: Option<&str>, raw: Option<&str>, enc: Option<&str>, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did = c_str!(submitter_did);
    let target_did = c_str!(target_did);

    let hash_str = opt_c_str!(hash);
    let raw_str = opt_c_str!(raw);
    let enc_str = opt_c_str!(enc);

    ErrorCode::from(unsafe {
        ledger::indy_build_attrib_request(command_handle,
                                          submitter_did.as_ptr(),
                                          target_did.as_ptr(),
                                          opt_c_ptr!(hash, hash_str),
                                          opt_c_ptr!(raw, raw_str),
                                          opt_c_ptr!(enc, enc_str),
                                          cb)
    })
}

/// Builds a GET_ATTRIB request. Request to get information about an Attribute for the specified DID.
///
/// # Arguments
/// * `submitter_did` - DID of the read request sender.
/// * `target_did` - Target DID as base58-encoded string for 16 or 32 bit DID value.
/// * `raw` - (Optional) Requested attribute name.
/// * `hash` - (Optional) Requested attribute hash.
/// * `enc` - (Optional) Requested attribute encrypted value.
///
/// # Returns
/// Request result as json.
pub fn build_get_attrib_request(submitter_did: Option<&str>, target_did: &str, raw: Option<&str>, hash: Option<&str>, enc: Option<&str>) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_get_attrib_request(command_handle, submitter_did, target_did, raw, hash, enc, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_get_attrib_request(command_handle: CommandHandle, submitter_did: Option<&str>, target_did: &str, raw: Option<&str>, hash: Option<&str>, enc: Option<&str>, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did_str = opt_c_str!(submitter_did);
    let target_did = c_str!(target_did);

    let raw_str = opt_c_str!(raw);
    let hash_str = opt_c_str!(hash);
    let enc_str = opt_c_str!(enc);

    ErrorCode::from(unsafe {
        ledger::indy_build_get_attrib_request(command_handle,
                                              opt_c_ptr!(submitter_did, submitter_did_str),
                                              target_did.as_ptr(),
                                              opt_c_ptr!(raw, raw_str),
                                              opt_c_ptr!(hash, hash_str),
                                              opt_c_ptr!(enc, enc_str),
                                              cb)
    })
}

/// Builds a SCHEMA request. Request to add Credential's schema.
///
/// # Arguments
/// * `submitter_did` - Identifier (DID) of the transaction author as base58-encoded string.
///                Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
/// * `data` - Credential schema.
/// {
///     id: identifier of schema
///     attrNames: array of attribute name strings (the number of attributes should be less or equal than 125)
///     name: Schema's name string
///     version: Schema's version string,
///     ver: Version of the Schema json
/// }
///
/// # Returns
/// Request result as json.
pub fn build_schema_request(submitter_did: &str, data: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_schema_request(command_handle, submitter_did, data, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_schema_request(command_handle: CommandHandle, submitter_did: &str, data: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did = c_str!(submitter_did);
    let data = c_str!(data);

    ErrorCode::from(unsafe { ledger::indy_build_schema_request(command_handle, submitter_did.as_ptr(), data.as_ptr(), cb) })
}

/// Builds a GET_SCHEMA request. Request to get Credential's Schema.
///
/// # Arguments
/// * `submitter_did` - (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
/// * `id` - Schema ID in ledger
///
/// # Returns
/// Request result as json.
pub fn build_get_schema_request(submitter_did: Option<&str>, id: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_get_schema_request(command_handle, submitter_did, id, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_get_schema_request(command_handle: CommandHandle, submitter_did: Option<&str>, id: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did_str = opt_c_str!(submitter_did);
    let id = c_str!(id);

    ErrorCode::from(unsafe { ledger::indy_build_get_schema_request(command_handle, opt_c_ptr!(submitter_did, submitter_did_str), id.as_ptr(), cb) })
}

/// Parse a GET_SCHEMA response to get Schema in the format compatible with Anoncreds API.
///
/// # Arguments
/// * `get_schema_response` - response of GET_SCHEMA request.
///
/// # Returns
/// Schema Id and Schema json.
/// {
///     id: identifier of schema
///     attrNames: array of attribute name strings
///     name: Schema's name string
///     version: Schema's version string
///     ver: Version of the Schema json
/// }
pub fn parse_get_schema_response(get_schema_response: &str) -> Box<dyn Future<Item=(String, String), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

    let err = _parse_get_schema_response(command_handle, get_schema_response, cb);

    ResultHandler::str_str(command_handle, err, receiver)
}

fn _parse_get_schema_response(command_handle: CommandHandle, get_schema_response: &str, cb: Option<ResponseStringStringCB>) -> ErrorCode {
    let get_schema_response = c_str!(get_schema_response);

    ErrorCode::from(unsafe { ledger::indy_parse_get_schema_response(command_handle, get_schema_response.as_ptr(), cb) })
}

/// Builds an CRED_DEF request. Request to add a Credential Definition (in particular, public key),
/// that Issuer creates for a particular Credential Schema.
///
/// # Arguments
/// * `submitter_did` - Identifier (DID) of the transaction author as base58-encoded string.
///                Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
/// * `data` - credential definition json
/// {
///     id: string - identifier of credential definition
///     schemaId: string - identifier of stored in ledger schema
///     type: string - type of the credential definition. CL is the only supported type now.
///     tag: string - allows to distinct between credential definitions for the same issuer and schema
///     value: Dictionary with Credential Definition's data: {
///         primary: primary credential public key,
///         Optional<revocation>: revocation credential public key
///     },
///     ver: Version of the CredDef json
/// }
///
/// # Returns
/// Request result as json.
pub fn build_cred_def_request(submitter_did: &str, data: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_cred_def_request(command_handle, submitter_did, data, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_cred_def_request(command_handle: CommandHandle, submitter_did: &str, data: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did = c_str!(submitter_did);
    let data = c_str!(data);

    ErrorCode::from(unsafe { ledger::indy_build_cred_def_request(command_handle, submitter_did.as_ptr(), data.as_ptr(), cb) })
}

/// Builds a GET_CRED_DEF request. Request to get a Credential Definition (in particular, public key),
/// that Issuer creates for a particular Credential Schema.
///
/// # Arguments
/// * `submitter_did` - (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
/// * `id` - Credential Definition ID in ledger.
///
/// # Returns
/// Request result as json.
pub fn build_get_cred_def_request(submitter_did: Option<&str>, id: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_get_cred_def_request(command_handle, submitter_did, id, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_get_cred_def_request(command_handle: CommandHandle, submitter_did: Option<&str>, id: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did_str = opt_c_str!(submitter_did);
    let id = c_str!(id);

    ErrorCode::from(unsafe { ledger::indy_build_get_cred_def_request(command_handle, opt_c_ptr!(submitter_did, submitter_did_str), id.as_ptr(), cb) })
}

/// Parse a GET_CRED_DEF response to get Credential Definition in the format compatible with Anoncreds API.
///
/// # Arguments
/// * `get_cred_def_response` - response of GET_CRED_DEF request.
///
/// # Returns
/// Credential Definition Id and Credential Definition json.
/// {
///     id: string - identifier of credential definition
///     schemaId: string - identifier of stored in ledger schema
///     type: string - type of the credential definition. CL is the only supported type now.
///     tag: string - allows to distinct between credential definitions for the same issuer and schema
///     value: Dictionary with Credential Definition's data: {
///         primary: primary credential public key,
///         Optional<revocation>: revocation credential public key
///     },
///     ver: Version of the Credential Definition json
/// }
pub fn parse_get_cred_def_response(get_cred_def_response: &str) -> Box<dyn Future<Item=(String, String), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

    let err = _parse_get_cred_def_response(command_handle, get_cred_def_response, cb);

    ResultHandler::str_str(command_handle, err, receiver)
}

fn _parse_get_cred_def_response(command_handle: CommandHandle, get_cred_def_response: &str, cb: Option<ResponseStringStringCB>) -> ErrorCode {
    let get_cred_def_response = c_str!(get_cred_def_response);

    ErrorCode::from(unsafe { ledger::indy_parse_get_cred_def_response(command_handle, get_cred_def_response.as_ptr(), cb) })
}

/// Builds a NODE request. Request to add a new node to the pool, or updates existing in the pool.
///
/// # Arguments
/// * `submitter_did` - Identifier (DID) of the transaction author as base58-encoded string.
///                Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
/// * `target_did` - Target Node's DID.  It differs from submitter_did field.
/// * `data` - Data associated with the Node: {
///     alias: string - Node's alias
///     blskey: string - (Optional) BLS multi-signature key as base58-encoded string.
///     client_ip: string - (Optional) Node's client listener IP address.
///     client_port: string - (Optional) Node's client listener port.
///     node_ip: string - (Optional) The IP address other Nodes use to communicate with this Node.
///     node_port: string - (Optional) The port other Nodes use to communicate with this Node.
///     services: array<string> - (Optional) The service of the Node. VALIDATOR is the only supported one now.
/// }
///
/// # Returns
/// Request result as json.
pub fn build_node_request(submitter_did: &str, target_did: &str, data: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_node_request(command_handle, submitter_did, target_did, data, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_node_request(command_handle: CommandHandle, submitter_did: &str, target_did: &str, data: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did = c_str!(submitter_did);
    let target_did = c_str!(target_did);
    let data = c_str!(data);

    ErrorCode::from(unsafe { ledger::indy_build_node_request(command_handle, submitter_did.as_ptr(), target_did.as_ptr(), data.as_ptr(), cb) })
}

/// Builds a GET_VALIDATOR_INFO request.
///
/// # Arguments
/// * `submitter_did` - Id of Identity stored in secured Wallet.
///
/// # Returns
/// Request result as json.
pub fn build_get_validator_info_request(submitter_did: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_get_validator_info_request(command_handle, submitter_did, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_get_validator_info_request(command_handle: CommandHandle, submitter_did: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did = c_str!(submitter_did);

    ErrorCode::from(unsafe {
        ledger::indy_build_get_validator_info_request(command_handle, submitter_did.as_ptr(), cb)
    })
}

/// Builds a POOL_CONFIG request. Request to change Pool's configuration.
///
/// # Arguments
/// * `submitter_did` - Identifier (DID) of the transaction author as base58-encoded string.
///                Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
/// * `writes` - Whether any write requests can be processed by the pool
///         (if false, then pool goes to read-only state). True by default.
/// * `force` - Whether we should apply transaction (for example, move pool to read-only state)
///        without waiting for consensus of this transaction.
///
/// # Returns
/// Request result as json.
pub fn build_pool_config_request(submitter_did: &str, writes: bool, force: bool) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_pool_config_request(command_handle, submitter_did, writes, force, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_pool_config_request(command_handle: CommandHandle, submitter_did: &str, writes: bool, force: bool, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did = c_str!(submitter_did);

    ErrorCode::from(unsafe { ledger::indy_build_pool_config_request(command_handle, submitter_did.as_ptr(), writes, force, cb) })
}

/// Builds a POOL_RESTART request.
///
/// # Arguments
/// * `submitter_did` - Identifier (DID) of the transaction author as base58-encoded string.
///                Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
/// * `action`-
/// * `datetime`-
///
/// # Returns
/// Request result as json.
pub fn build_pool_restart_request(submitter_did: &str, action: &str, datetime: Option<&str>) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_pool_restart_request(command_handle, submitter_did, action, datetime, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_pool_restart_request(command_handle: CommandHandle, submitter_did: &str, action: &str, datetime: Option<&str>, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did = c_str!(submitter_did);
    let action = c_str!(action);
    let datetime_str = opt_c_str!(datetime);

    ErrorCode::from(unsafe {
        ledger::indy_build_pool_restart_request(command_handle,
                                                submitter_did.as_ptr(),
                                                action.as_ptr(),
                                                opt_c_ptr!(datetime, datetime_str),
                                                cb)
    })
}

/// Builds a POOL_UPGRADE request. Request to upgrade the Pool (sent by Trustee).
/// It upgrades the specified Nodes (either all nodes in the Pool, or some specific ones).
///
/// # Arguments
/// * `submitter_did` - Identifier (DID) of the transaction author as base58-encoded string.
///                Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
/// * `name` - Human-readable name for the upgrade.
/// * `version` - The version of indy-node package we perform upgrade to.
///          Must be greater than existing one (or equal if reinstall flag is True).
/// * `action` - Either start or cancel.
/// * `sha256` - sha256 hash of the package.
/// * `upgrade_timeout` - (Optional) Limits upgrade time on each Node.
/// * `schedule` - (Optional) Schedule of when to perform upgrade on each node. Map Node DIDs to upgrade time.
/// * `justification` - (Optional) justification string for this particular Upgrade.
/// * `reinstall` - Whether it's allowed to re-install the same version. False by default.
/// * `force` - Whether we should apply transaction (schedule Upgrade) without waiting
///        for consensus of this transaction.
///
/// # Returns
/// Request result as json.
pub fn build_pool_upgrade_request(submitter_did: &str,
                                  name: &str,
                                  version: &str,
                                  action: &str,
                                  sha256: &str,
                                  upgrade_timeout: Option<u32>,
                                  schedule: Option<&str>,
                                  justification: Option<&str>,
                                  reinstall: bool,
                                  force: bool,
                                  package: Option<&str>) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_pool_upgrade_request(command_handle, submitter_did, name, version, action, sha256, upgrade_timeout, schedule, justification, reinstall, force, package, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_pool_upgrade_request(command_handle: CommandHandle,
                               submitter_did: &str,
                               name: &str,
                               version: &str,
                               action: &str,
                               sha256: &str,
                               upgrade_timeout: Option<u32>,
                               schedule: Option<&str>,
                               justification: Option<&str>,
                               reinstall: bool,
                               force: bool,
                               package: Option<&str>,
                               cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did = c_str!(submitter_did);
    let name = c_str!(name);
    let version = c_str!(version);
    let action = c_str!(action);
    let sha256 = c_str!(sha256);
    let upgrade_timeout = upgrade_timeout.map(|t| t as i32).unwrap_or(-1);

    let schedule_str = opt_c_str!(schedule);
    let justification_str = opt_c_str!(justification);
    let package_str = opt_c_str!(package);

    ErrorCode::from(unsafe {
        ledger::indy_build_pool_upgrade_request(command_handle,
                                                submitter_did.as_ptr(),
                                                name.as_ptr(),
                                                version.as_ptr(),
                                                action.as_ptr(),
                                                sha256.as_ptr(),
                                                upgrade_timeout,
                                                opt_c_ptr!(schedule, schedule_str),
                                                opt_c_ptr!(justification, justification_str),
                                                reinstall,
                                                force,
                                                opt_c_ptr!(package, package_str),
                                                cb)
    })
}

/// Builds a REVOC_REG_DEF request. Request to add the definition of revocation registry
/// to an exists credential definition.
///
/// # Arguments
/// * `submitter_did` - Identifier (DID) of the transaction author as base58-encoded string.
///                Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
/// * `data` - Revocation Registry data:
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
///             "publicKeys": <public_keys> - Registry's public key.
///         },
///         "ver": string - version of revocation registry definition json.
///     }
///
/// # Returns
/// Request result as json.
pub fn build_revoc_reg_def_request(submitter_did: &str, data: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_revoc_reg_def_request(command_handle, submitter_did, data, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_revoc_reg_def_request(command_handle: CommandHandle, submitter_did: &str, data: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did = c_str!(submitter_did);
    let data = c_str!(data);

    ErrorCode::from(unsafe { ledger::indy_build_revoc_reg_def_request(command_handle, submitter_did.as_ptr(), data.as_ptr(), cb) })
}

/// Builds a GET_REVOC_REG_DEF request. Request to get a revocation registry definition,
/// that Issuer creates for a particular Credential Definition.
///
/// # Arguments
/// * `submitter_did` - (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
/// * `id` -  ID of Revocation Registry Definition in ledger.
///
/// # Returns
/// Request result as json.
pub fn build_get_revoc_reg_def_request(submitter_did: Option<&str>, id: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_get_revoc_reg_def_request(command_handle, submitter_did, id, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_get_revoc_reg_def_request(command_handle: CommandHandle, submitter_did: Option<&str>, id: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did_str = opt_c_str!(submitter_did);
    let id = c_str!(id);

    ErrorCode::from(unsafe { ledger::indy_build_get_revoc_reg_def_request(command_handle, opt_c_ptr!(submitter_did, submitter_did_str), id.as_ptr(), cb) })
}

/// Parse a GET_REVOC_REG_DEF response to get Revocation Registry Definition in the format
/// compatible with Anoncreds API.
///
/// #Params
/// * `get_revoc_reg_def_response` - response of GET_REVOC_REG_DEF request.
///
/// # Returns
/// Revocation Registry Definition Id and Revocation Registry Definition json.
/// {
///     "id": string - ID of the Revocation Registry,
///     "revocDefType": string - Revocation Registry type (only CL_ACCUM is supported for now),
///     "tag": string - Unique descriptive ID of the Registry,
///     "credDefId": string - ID of the corresponding CredentialDefinition,
///     "value": Registry-specific data {
///         "issuanceType": string - Type of Issuance(ISSUANCE_BY_DEFAULT or ISSUANCE_ON_DEMAND),
///         "maxCredNum": number - Maximum number of credentials the Registry can serve.
///         "tailsHash": string - Hash of tails.
///         "tailsLocation": string - Location of tails file.
///         "publicKeys": <public_keys> - Registry's public key.
///     },
///     "ver": string - version of revocation registry definition json.
/// }
pub fn parse_get_revoc_reg_def_response(get_revoc_reg_def_response: &str) -> Box<dyn Future<Item=(String, String), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

    let err = _parse_get_revoc_reg_def_response(command_handle, get_revoc_reg_def_response, cb);

    ResultHandler::str_str(command_handle, err, receiver)
}

fn _parse_get_revoc_reg_def_response(command_handle: CommandHandle, get_revoc_reg_def_response: &str, cb: Option<ResponseStringStringCB>) -> ErrorCode {
    let get_revoc_reg_def_response = c_str!(get_revoc_reg_def_response);

    ErrorCode::from(unsafe { ledger::indy_parse_get_revoc_reg_def_response(command_handle, get_revoc_reg_def_response.as_ptr(), cb) })
}

/// Builds a REVOC_REG_ENTRY request.  Request to add the RevocReg entry containing
/// the new accumulator value and issued/revoked indices.
/// This is just a delta of indices, not the whole list.
/// So, it can be sent each time a new credential is issued/revoked.
///
/// # Arguments
/// * `submitter_did` - Identifier (DID) of the transaction author as base58-encoded string.
///                Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
/// * `revoc_reg_def_id` - ID of the corresponding RevocRegDef.
/// * `rev_def_type` - Revocation Registry type (only CL_ACCUM is supported for now).
/// * `value` - Registry-specific data: {
///     value: {
///         prevAccum: string - previous accumulator value.
///         accum: string - current accumulator value.
///         issued: array<number> - an array of issued indices.
///         revoked: array<number> an array of revoked indices.
///     },
///     ver: string - version revocation registry entry json
///
/// }
///
/// # Returns
/// Request result as json.
pub fn build_revoc_reg_entry_request(submitter_did: &str, revoc_reg_def_id: &str, rev_def_type: &str, value: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_revoc_reg_entry_request(command_handle, submitter_did, revoc_reg_def_id, rev_def_type, value, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_revoc_reg_entry_request(command_handle: CommandHandle, submitter_did: &str, revoc_reg_def_id: &str, rev_def_type: &str, value: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did = c_str!(submitter_did);
    let revoc_reg_def_id = c_str!(revoc_reg_def_id);
    let rev_def_type = c_str!(rev_def_type);
    let value = c_str!(value);

    ErrorCode::from(unsafe { ledger::indy_build_revoc_reg_entry_request(command_handle, submitter_did.as_ptr(), revoc_reg_def_id.as_ptr(), rev_def_type.as_ptr(), value.as_ptr(), cb) })
}

/// Builds a GET_REVOC_REG request. Request to get the accumulated state of the Revocation Registry
/// by ID. The state is defined by the given timestamp.
///
/// # Arguments
/// * `submitter_did` - (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
/// * `revoc_reg_def_id` -  ID of the corresponding Revocation Registry Definition in ledger.
/// * `timestamp` - Requested time represented as a total number of seconds from Unix Epoch
///
/// # Returns
/// Request result as json.
pub fn build_get_revoc_reg_request(submitter_did: Option<&str>, revoc_reg_def_id: &str, timestamp: i64) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_get_revoc_reg_request(command_handle, submitter_did, revoc_reg_def_id, timestamp, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_get_revoc_reg_request(command_handle: CommandHandle, submitter_did: Option<&str>, revoc_reg_def_id: &str, timestamp: i64, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did_str = opt_c_str!(submitter_did);
    let revoc_reg_def_id = c_str!(revoc_reg_def_id);

    ErrorCode::from(unsafe { ledger::indy_build_get_revoc_reg_request(command_handle, opt_c_ptr!(submitter_did, submitter_did_str), revoc_reg_def_id.as_ptr(), timestamp, cb) })
}

/// Parse a GET_REVOC_REG response to get Revocation Registry in the format compatible with Anoncreds API.
///
/// # Arguments
/// * `get_revoc_reg_response` - response of GET_REVOC_REG request.
///
/// # Returns
/// Revocation Registry Definition Id, Revocation Registry json and Timestamp.
/// {
///     "value": Registry-specific data {
///         "accum": string - current accumulator value.
///     },
///     "ver": string - version revocation registry json
/// }
pub fn parse_get_revoc_reg_response(get_revoc_reg_response: &str) -> Box<dyn Future<Item=(String, String, u64), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string_u64();

    let err = _parse_get_revoc_reg_response(command_handle, get_revoc_reg_response, cb);

    ResultHandler::str_str_u64(command_handle, err, receiver)
}

fn _parse_get_revoc_reg_response(command_handle: CommandHandle, get_revoc_reg_response: &str, cb: Option<ResponseStringStringU64CB>) -> ErrorCode {
    let get_revoc_reg_response = c_str!(get_revoc_reg_response);

    ErrorCode::from(unsafe { ledger::indy_parse_get_revoc_reg_response(command_handle, get_revoc_reg_response.as_ptr(), cb) })
}

/// Builds a GET_REVOC_REG_DELTA request. Request to get the delta of the accumulated state of the Revocation Registry.
/// The Delta is defined by from and to timestamp fields.
/// If from is not specified, then the whole state till to will be returned.
///
/// # Arguments
/// * `submitter_did` - (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
/// * `revoc_reg_def_id` -  ID of the corresponding Revocation Registry Definition in ledger.
/// * `from` - Requested time represented as a total number of seconds from Unix Epoch
/// * `to` - Requested time represented as a total number of seconds from Unix Epoch
///
/// # Returns
/// Request result as json.
pub fn build_get_revoc_reg_delta_request(submitter_did: Option<&str>, revoc_reg_def_id: &str, from: i64, to: i64) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_get_revoc_reg_delta_request(command_handle, submitter_did, revoc_reg_def_id, from, to, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_get_revoc_reg_delta_request(command_handle: CommandHandle, submitter_did: Option<&str>, revoc_reg_def_id: &str, from: i64, to: i64, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did_str = opt_c_str!(submitter_did);
    let revoc_reg_def_id = c_str!(revoc_reg_def_id);

    ErrorCode::from(unsafe { ledger::indy_build_get_revoc_reg_delta_request(command_handle, opt_c_ptr!(submitter_did, submitter_did_str), revoc_reg_def_id.as_ptr(), from, to, cb) })
}

/// Parse a GET_REVOC_REG_DELTA response to get Revocation Registry Delta in the format compatible with Anoncreds API.
///
/// # Arguments
/// * `get_revoc_reg_response` - response of GET_REVOC_REG_DELTA request.
///
/// # Returns
/// Revocation Registry Definition Id, Revocation Registry Delta json and Timestamp.
/// {
///     "value": Registry-specific data {
///         prevAccum: string - previous accumulator value.
///         accum: string - current accumulator value.
///         issued: array<number> - an array of issued indices.
///         revoked: array<number> an array of revoked indices.
///     },
///     "ver": string - version revocation registry delta json
/// }
pub fn parse_get_revoc_reg_delta_response(get_revoc_reg_delta_response: &str) -> Box<dyn Future<Item=(String, String, u64), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string_u64();

    let err = _parse_get_revoc_reg_delta_response(command_handle, get_revoc_reg_delta_response, cb);

    ResultHandler::str_str_u64(command_handle, err, receiver)
}

fn _parse_get_revoc_reg_delta_response(command_handle: CommandHandle, get_revoc_reg_delta_response: &str, cb: Option<ResponseStringStringU64CB>) -> ErrorCode {
    let get_revoc_reg_delta_response = c_str!(get_revoc_reg_delta_response);

    ErrorCode::from(unsafe { ledger::indy_parse_get_revoc_reg_delta_response(command_handle, get_revoc_reg_delta_response.as_ptr(), cb) })
}

/// Parse transaction response to fetch metadata.
/// The important use case for this method is validation of Node's response freshens.
///
/// Distributed Ledgers can reply with outdated information for consequence read request after write.
/// To reduce pool load libindy sends read requests to one random node in the pool.
/// Consensus validation is performed based on validation of nodes multi signature for current ledger Merkle Trie root.
/// This multi signature contains information about the latest ldeger's transaction ordering time and sequence number that this method returns.
///
/// If node that returned response for some reason is out of consensus and has outdated ledger
/// it can be caught by analysis of the returned latest ledger's transaction ordering time and sequence number.
///
/// There are two ways to filter outdated responses:
///     1) based on "seqNo" - sender knows the sequence number of transaction that he consider as a fresh enough.
///     2) based on "txnTime" - sender knows the timestamp that he consider as a fresh enough.
///
/// Note: response of GET_VALIDATOR_INFO request isn't supported
///
/// # Arguments
/// * `response` - response of write or get request.
///
/// Note: response of GET_VALIDATOR_INFO request isn't supported
///
/// # Returns
/// response metadata
/// {
///     "seqNo": Option<u64> - transaction sequence number,
///     "txnTime": Option<u64> - transaction ordering time,
///     "lastSeqNo": Option<u64> - the latest transaction seqNo for particular Node,
///     "lastTxnTime": Option<u64> - the latest transaction ordering time for particular Node
/// }
pub fn get_response_metadata(response: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _get_response_metadata(command_handle, response, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _get_response_metadata(command_handle: CommandHandle, response: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let response = c_str!(response);

    ErrorCode::from(unsafe { ledger::indy_get_response_metadata(command_handle, response.as_ptr(), cb) })
}

/// Builds a AUTH_RULE request. Request to change authentication rules for a ledger transaction.
///
/// # Arguments
/// * `submitter_did` - Identifier (DID) of the transaction author as base58-encoded string.
///                Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
/// * `txn_type`: ledger transaction alias or associated value for which authentication rules will be applied.
/// * `field`: type of an action for which authentication rules will be applied.
///     Can be either "ADD" (to add a new rule) or "EDIT" (to edit an existing one).
/// * `action`: transaction field for which authentication rule will be applied.
/// * `old_value`: (Optional) old value of a field, which can be changed to a new_value (mandatory for EDIT action).
/// * `new_value`:(Optional) new value that can be used to fill the field.
/// * `constraint`: set of constraints required for execution of an action in the following format:
///     {
///         constraint_id - <string> type of a constraint.
///             Can be either "ROLE" to specify final constraint or  "AND"/"OR" to combine constraints.
///         role - <string> (optional) role of a user which satisfy to constrain.
///         sig_count - <u32> the number of signatures required to execution action.
///         need_to_be_owner - <bool> (optional) if user must be an owner of transaction (false by default).
///         off_ledger_signature - <bool> (optional) allow signature of unknow for ledger did (false by default).
///         metadata - <object> (optional) additional parameters of the constraint.
///     }
/// can be combined by
///     {
///         'constraint_id': <"AND" or "OR">
///         'auth_constraints': [<constraint_1>, <constraint_2>]
///     }
///
/// # Returns
/// Request result as json.
pub fn build_auth_rule_request(submitter_did: &str, txn_type: &str, action: &str, field: &str,
                               old_value: Option<&str>, new_value: Option<&str>, constraint: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_auth_rule_request(command_handle, submitter_did, txn_type, action, field, old_value, new_value, constraint, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_auth_rule_request(command_handle: CommandHandle,
                            submitter_did: &str,
                            txn_type: &str,
                            action: &str,
                            field: &str,
                            old_value: Option<&str>,
                            new_value: Option<&str>,
                            constraint: &str,
                            cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did = c_str!(submitter_did);
    let txn_type = c_str!(txn_type);
    let action = c_str!(action);
    let field = c_str!(field);
    let constraint = c_str!(constraint);

    let old_value_str = opt_c_str!(old_value);
    let new_value_str = opt_c_str!(new_value);

    ErrorCode::from(unsafe {
        ledger::indy_build_auth_rule_request(command_handle,
                                             submitter_did.as_ptr(),
                                             txn_type.as_ptr(),
                                             action.as_ptr(),
                                             field.as_ptr(),
                                             opt_c_ptr!(old_value, old_value_str),
                                             opt_c_ptr!(new_value, new_value_str),
                                             constraint.as_ptr(),
                                             cb)
    })
}

/// Builds a AUTH_RULES request. Request to change multiple authentication rules for a ledger transaction.
///
/// # Arguments
/// * `submitter_did` - Identifier (DID) of the transaction author as base58-encoded string.
///                Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
/// * `data`: a list of auth rules: [
///     {
///         "auth_type": ledger transaction alias or associated value,
///         "auth_action": type of an action,
///         "field": transaction field,
///         "old_value": (Optional) old value of a field, which can be changed to a new_value (mandatory for EDIT action),
///         "new_value": (Optional) new value that can be used to fill the field,
///         "constraint": set of constraints required for execution of an action in the format described above for `build_auth_rule_request` function.
///     }
/// ]
///
/// # Returns
/// Request result as json.
pub fn build_auth_rules_request(submitter_did: &str, data: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_auth_rules_request(command_handle, submitter_did, data, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_auth_rules_request(command_handle: CommandHandle,
                             submitter_did: &str,
                             data: &str,
                             cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did = c_str!(submitter_did);
    let data = c_str!(data);

    ErrorCode::from(unsafe {
        ledger::indy_build_auth_rules_request(command_handle,
                                              submitter_did.as_ptr(),
                                              data.as_ptr(),
                                              cb)
    })
}

/// Builds a GET_AUTH_RULE request. Request to get authentication rules for a ledger transaction.
///
/// NOTE: Either none or all transaction related parameters must be specified (`old_value` can be skipped for `ADD` action).
///     * none - to get all authentication rules for all ledger transactions
///     * all - to get authentication rules for specific action (`old_value` can be skipped for `ADD` action)
///
/// # Arguments
/// * `submitter_did` - (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
/// * `txn_type`: (Optional) target ledger transaction alias or associated value.
/// * `action`: (Optional) target action type. Can be either "ADD" or "EDIT".
/// * `field`: (Optional) target transaction field.
/// * `old_value`: (Optional) old value of field, which can be changed to a new_value (mandatory for EDIT action).
/// * `new_value`: (Optional) new value that can be used to fill the field.
///
/// # Returns
/// Request result as json.
pub fn build_get_auth_rule_request(submitter_did: Option<&str>, txn_type: Option<&str>, action: Option<&str>, field: Option<&str>,
                                   old_value: Option<&str>, new_value: Option<&str>) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_get_auth_rule_request(command_handle, submitter_did, txn_type, action, field, old_value, new_value, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_get_auth_rule_request(command_handle: CommandHandle,
                                submitter_did: Option<&str>,
                                txn_type: Option<&str>,
                                action: Option<&str>,
                                field: Option<&str>,
                                old_value: Option<&str>,
                                new_value: Option<&str>,
                                cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did_str = opt_c_str!(submitter_did);

    let txn_type_str = opt_c_str!(txn_type);
    let action_str = opt_c_str!(action);
    let field_str = opt_c_str!(field);
    let new_value_str = opt_c_str!(new_value);

    let old_value_str = opt_c_str!(old_value);

    ErrorCode::from(unsafe {
        ledger::indy_build_get_auth_rule_request(command_handle,
                                                 opt_c_ptr!(submitter_did, submitter_did_str),
                                                 opt_c_ptr!(txn_type, txn_type_str),
                                                 opt_c_ptr!(action, action_str),
                                                 opt_c_ptr!(field, field_str),
                                                 opt_c_ptr!(old_value, old_value_str),
                                                 opt_c_ptr!(new_value, new_value_str),
                                                 cb)
    })
}

/// Builds a TXN_AUTHR_AGRMT request. Request to add a new version of Transaction Author Agreement to the ledger.
///
/// # Arguments
/// * `submitter_did` - Identifier (DID) of the transaction author as base58-encoded string.
///                Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
/// * `text`: (Optional) a content of the TTA.
///            Mandatory in case of adding a new TAA. An existing TAA text can not be changed.
///             for Indy Node version <= 1.12.0:
///                 Use empty string to reset TAA on the ledger
///             for Indy Node version > 1.12.0
///                 Should be omitted in case of updating an existing TAA (setting `retirement_ts`)
/// * `version`: a version of the TTA (unique UTF-8 string).
/// * `ratification_ts`: (Optional) the date (timestamp) of TAA ratification by network government.
///              for Indy Node version <= 1.12.0:
///                 Must be omitted
///              for Indy Node version > 1.12.0:
///                 Must be specified in case of adding a new TAA
///                 Can be omitted in case of updating an existing TAA
/// * `retirement_ts`: (Optional) the date (timestamp) of TAA retirement.
///              for Indy Node version <= 1.12.0:
///                 Must be omitted
///              for Indy Node version > 1.12.0:
///                 Must be omitted in case of adding a new (latest) TAA.
///                 Should be used for updating (deactivating) non-latest TAA on the ledger.
///
/// Note: Use `build_disable_all_txn_author_agreements_request` to disable all TAA's on the ledger.
///
/// # Returns
/// Request result as json.
pub fn build_txn_author_agreement_request(submitter_did: &str, text: Option<&str>, version: &str, ratification_ts: Option<u64>, retirement_ts: Option<u64>) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_txn_author_agreement_request(command_handle, submitter_did, text, version, ratification_ts, retirement_ts, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_txn_author_agreement_request(command_handle: CommandHandle,
                                       submitter_did: &str,
                                       text: Option<&str>,
                                       version: &str,
                                       ratification_ts: Option<u64>,
                                       retirement_ts: Option<u64>,
                                       cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did = c_str!(submitter_did);
    let text_str = opt_c_str!(text);
    let ratification_ts = opt_u64!(ratification_ts);
    let retirement_ts = opt_u64!(retirement_ts);
    let version = c_str!(version);

    ErrorCode::from(unsafe {
        ledger::indy_build_txn_author_agreement_request(command_handle,
                                                        submitter_did.as_ptr(),
                                                        opt_c_ptr!(text, text_str),
                                                        version.as_ptr(),
                                                        ratification_ts,
                                                        retirement_ts,
                                                        cb)
    })
}

/// Builds a DISABLE_ALL_TXN_AUTHR_AGRMTS request. Request to disable all Transaction Author Agreement on the ledger.
///
/// # Arguments
/// * `submitter_did` - Identifier (DID) of the transaction author as base58-encoded string.
///                Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
///
/// # Returns
/// Request result as json.
pub fn build_disable_all_txn_author_agreements_request(submitter_did: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_disable_all_txn_author_agreements_request(command_handle, submitter_did, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_disable_all_txn_author_agreements_request(command_handle: CommandHandle,
                                                    submitter_did: &str,
                                                    cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did = c_str!(submitter_did);

    ErrorCode::from(unsafe {
        ledger::indy_build_disable_all_txn_author_agreements_request(command_handle,
                                                                     submitter_did.as_ptr(),
                                                                     cb)
    })
}

/// Builds a GET_TXN_AUTHR_AGRMT request. Request to get a specific Transaction Author Agreement from the ledger.
///
/// # Arguments
/// * `submitter_did` - (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
/// * `data`: (Optional) specifies a condition for getting specific TAA.
/// Contains 3 mutually exclusive optional fields:
/// {
///     hash: Optional<str> - hash of requested TAA,
///     version: Optional<str> - version of requested TAA.
///     timestamp: Optional<u64> - ledger will return TAA valid at requested timestamp.
/// }
/// Null data or empty JSON are acceptable here. In this case, ledger will return the latest version of TAA.
///
/// # Returns
/// Request result as json.
pub fn build_get_txn_author_agreement_request(submitter_did: Option<&str>, data: Option<&str>) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_get_txn_author_agreement_request(command_handle, submitter_did, data, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_get_txn_author_agreement_request(command_handle: CommandHandle,
                                           submitter_did: Option<&str>,
                                           data: Option<&str>,
                                           cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did_str = opt_c_str!(submitter_did);
    let data_str = opt_c_str!(data);

    ErrorCode::from(unsafe {
        ledger::indy_build_get_txn_author_agreement_request(command_handle,
                                                            opt_c_ptr!(submitter_did, submitter_did_str),
                                                            opt_c_ptr!(data, data_str),
                                                            cb)
    })
}

/// Builds a SET_TXN_AUTHR_AGRMT_AML request. Request to add a new list of acceptance mechanisms for transaction author agreement.
/// Acceptance Mechanism is a description of the ways how the user may accept a transaction author agreement.
///
/// # Arguments
/// * `submitter_did` - Identifier (DID) of the transaction author as base58-encoded string.
///                Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
/// * `aml`: a set of new acceptance mechanisms:
/// {
///     “<acceptance mechanism label 1>”: { acceptance mechanism description 1},
///     “<acceptance mechanism label 2>”: { acceptance mechanism description 2},
///     ...
/// }
/// * `version`: a version of new acceptance mechanisms. (Note: unique on the Ledger).
/// * `aml_context`: (Optional) common context information about acceptance mechanisms (may be a URL to external resource).
///
/// # Returns
/// Request result as json.
pub fn build_acceptance_mechanisms_request(submitter_did: &str, aml: &str, version: &str, aml_context: Option<&str>) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_acceptance_mechanisms_request(command_handle, submitter_did, aml, version, aml_context, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_acceptance_mechanisms_request(command_handle: CommandHandle,
                                        submitter_did: &str,
                                        aml: &str,
                                        version: &str,
                                        aml_context: Option<&str>,
                                        cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did = c_str!(submitter_did);
    let aml = c_str!(aml);
    let version = c_str!(version);
    let aml_context_str = opt_c_str!(aml_context);

    ErrorCode::from(unsafe {
        ledger::indy_build_acceptance_mechanisms_request(command_handle,
                                                         submitter_did.as_ptr(),
                                                         aml.as_ptr(),
                                                         version.as_ptr(),
                                                         opt_c_ptr!(aml_context, aml_context_str),
                                                         cb)
    })
}

/// Builds a GET_TXN_AUTHR_AGRMT_AML request. Request to get a list of  acceptance mechanisms from the ledger
/// valid for specified time or the latest one.
///
/// # Arguments
/// * `submitter_did` - (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
/// * `timestamp`: (Optional) time to get an active acceptance mechanisms.
/// * `version`: (Optional) version of acceptance mechanisms.
///
/// NOTE: timestamp and version cannot be specified together.
///
/// # Returns
/// Request result as json.
pub fn build_get_acceptance_mechanisms_request(submitter_did: Option<&str>, timestamp: Option<i64>, version: Option<&str>) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_get_acceptance_mechanisms_request(command_handle, submitter_did, timestamp, version, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_get_acceptance_mechanisms_request(command_handle: CommandHandle,
                                            submitter_did: Option<&str>,
                                            timestamp: Option<i64>,
                                            version: Option<&str>,
                                            cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did_str = opt_c_str!(submitter_did);
    let timestamp = timestamp.unwrap_or(-1);
    let version_str = opt_c_str!(version);

    ErrorCode::from(unsafe {
        ledger::indy_build_get_acceptance_mechanisms_request(command_handle,
                                                             opt_c_ptr!(submitter_did, submitter_did_str),
                                                             timestamp,
                                                             opt_c_ptr!(version, version_str),
                                                             cb)
    })
}

/// Append transaction author agreement acceptance data to a request.
/// This function should be called before signing and sending a request
/// if there is any transaction author agreement set on the Ledger.
///
/// This function may calculate digest by itself or consume it as a parameter.
/// If all text, version and taa_digest parameters are specified, a check integrity of them will be done.
///
/// # Arguments
/// * `request_json`: original request data json.
/// * `text` and `version`: (optional) raw data about TAA from ledger.
///     These parameters should be passed together.
///     These parameters are required if taa_digest parameter is omitted.
/// * `taa_digest`: (optional) digest on text and version.
///     Digest is sha256 hash calculated on concatenated strings: version || text.
///     This parameter is required if text and version parameters are omitted.
/// * `mechanism`: mechanism how user has accepted the TAA
/// * `time`: UTC timestamp when user has accepted the TAA. Note that the time portion will be discarded to avoid a privacy risk.
///
/// # Returns
/// Updated request result as json.
pub fn append_txn_author_agreement_acceptance_to_request(request_json: &str,
                                                         text: Option<&str>,
                                                         version: Option<&str>,
                                                         taa_digest: Option<&str>,
                                                         mechanism: &str,
                                                         time: u64) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _append_txn_author_agreement_acceptance_to_request(command_handle, request_json, text, version, taa_digest, mechanism, time, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _append_txn_author_agreement_acceptance_to_request(command_handle: CommandHandle,
                                                      request_json: &str,
                                                      text: Option<&str>,
                                                      version: Option<&str>,
                                                      taa_digest: Option<&str>,
                                                      mechanism: &str,
                                                      time: u64,
                                                      cb: Option<ResponseStringCB>) -> ErrorCode {
    let request_json = c_str!(request_json);
    let text_str = opt_c_str!(text);
    let version_str = opt_c_str!(version);
    let taa_digest_str = opt_c_str!(taa_digest);
    let mechanism = c_str!(mechanism);

    ErrorCode::from(unsafe {
        ledger::indy_append_txn_author_agreement_acceptance_to_request(command_handle,
                                                                       request_json.as_ptr(),
                                                                       opt_c_ptr!(text, text_str),
                                                                       opt_c_ptr!(version, version_str),
                                                                       opt_c_ptr!(taa_digest, taa_digest_str),
                                                                       mechanism.as_ptr(),
                                                                       time,
                                                                       cb)
    })
}

/// Append Endorser to an existing request.
///
/// An author of request still is a `DID` used as a `submitter_did` parameter for the building of the request.
/// But it is expecting that the transaction will be sent by the specified Endorser.
///
/// Note: Both Transaction Author and Endorser must sign output request after that.
///
/// More about Transaction Endorser: https://github.com/hyperledger/indy-node/blob/master/design/transaction_endorser.md
///                                  https://github.com/hyperledger/indy-sdk/blob/master/docs/configuration.md
///
/// # Arguments
/// * `request_json`: original request data json.
/// * `endorser_did`: DID of the Endorser that will submit the transaction.
///                   The Endorser's DID must be present on the ledger.
/// # Returns
/// Updated request result as json.
pub fn append_request_endorser(request_json: &str,
                               endorser_did: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _append_request_endorser(command_handle, request_json, endorser_did, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _append_request_endorser(command_handle: CommandHandle,
                            request_json: &str,
                            endorser_did: &str,
                            cb: Option<ResponseStringCB>) -> ErrorCode {
    let request_json = c_str!(request_json);
    let endorser_did = c_str!(endorser_did);

    ErrorCode::from(unsafe {
        ledger::indy_append_request_endorser(command_handle,
                                             request_json.as_ptr(),
                                             endorser_did.as_ptr(),
                                             cb)
    })
}
/// Request to freeze list of ledgers.
///
/// # Arguments
/// * `command_handle`: command handle to map callback to caller context.
/// * `submitter_did`: (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
/// * `ledgers_ids`: list of ledgers IDs for freezing.
/// * `cb`: Callback that takes command result as parameter.
///
/// # Returns
/// Updated request result as json.
pub fn build_ledgers_freeze_request(submitter_did: &str, ledgers_ids: Vec<u64>) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();
    let json_ledgers_ids: &str = &json!(ledgers_ids).to_string();
    let err = _build_ledgers_freeze_request(command_handle, submitter_did, json_ledgers_ids, cb);
    ResultHandler::str(command_handle, err, receiver)
}

fn _build_ledgers_freeze_request(command_handle: CommandHandle, submitter_did: &str, ledgers_ids: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did = c_str!(submitter_did);
    let ledgers_ids = c_str!(ledgers_ids);

    ErrorCode::from(unsafe {
        ledger::indy_build_ledgers_freeze_request(command_handle,
                                                submitter_did.as_ptr(),
                                                ledgers_ids.as_ptr(),
                                                cb)
    })
}

/// Request to get list of frozen ledgers.
///
/// # Arguments
/// * `command_handle`: command handle to map callback to caller context.
/// * `submitter_did`: (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
/// * `cb`: Callback that takes command result as parameter.
///
/// # Returns
/// Request result as json.
///  {
///     <ledger_id>: {
///         "ledger": String - Ledger root hash,
///         "state": String - State root hash,
///         "seq_no": u64 - the latest transaction seqNo for particular Node,
///     },
///     ...
/// }
pub fn build_get_frozen_ledgers_request(submitter_did: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();
    let err = _build_get_frozen_ledgers_request(command_handle, submitter_did, cb);
    ResultHandler::str(command_handle, err, receiver)
}

fn _build_get_frozen_ledgers_request(command_handle: CommandHandle, submitter_did: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did = c_str!(submitter_did);

    ErrorCode::from(unsafe {
        ledger::indy_build_get_frozen_ledgers_request(command_handle,
                                             submitter_did.as_ptr(),
                                             cb)
    })
}