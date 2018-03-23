extern crate libc;

use api::ErrorCode;
use errors::ToErrorCode;
use commands::{Command, CommandExecutor};
use commands::anoncreds::AnoncredsCommand;
use commands::anoncreds::issuer::IssuerCommand;
use commands::anoncreds::prover::ProverCommand;
use commands::anoncreds::verifier::VerifierCommand;
use utils::cstring::CStringUtils;

use self::libc::c_char;

/// Create keys (both primary and revocation) for the given schema and signature type (currently only CL signature type is supported).
/// Store the keys together with signature type and schema in a secure wallet as a claim definition.
/// The claim definition in the wallet is identifying by a returned unique key.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// issuer_did: a DID of the issuer signing claim_def transaction to the Ledger
/// schema_json: schema as a json
/// signature_type: signature type (optional). Currently only 'CL' is supported.
/// create_non_revoc: whether to request non-revocation claim.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// claim definition json containing information about signature type, schema and issuer's public key.
/// Unique number identifying the public key in the wallet
///
/// #Errors
/// Common*
/// Wallet*
/// Anoncreds*
#[no_mangle]
pub extern "C" fn indy_issuer_create_and_store_claim_def(
    command_handle: i32,
    wallet_handle: i32,
    issuer_did: *const c_char,
    schema_json: *const c_char,
    signature_type: *const c_char,
    create_non_revoc: bool,
    cb: Option<
        extern "C" fn(xcommand_handle: i32,
                      err: ErrorCode,
                      claim_def_json: *const c_char),
    >,
) -> ErrorCode {
    check_useful_c_str!(issuer_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(schema_json, ErrorCode::CommonInvalidParam4);
    check_useful_opt_c_str!(signature_type, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    let result = CommandExecutor::instance().send(Command::Anoncreds(AnoncredsCommand::Issuer(
        IssuerCommand::CreateAndStoreClaimDefinition(
            wallet_handle,
            issuer_did,
            schema_json,
            signature_type,
            create_non_revoc,
            Box::new(move |result| {
                let (err, claim_def_json) = result_to_err_code_1!(result, String::new());
                let claim_def_json = CStringUtils::string_to_cstring(claim_def_json);
                cb(command_handle, err, claim_def_json.as_ptr())
            }),
        ),
    )));

    result_to_err_code!(result)
}

/// Create a new revocation registry for the given claim definition.
/// Stores it in a secure wallet identifying by the returned key.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// issuer_did: a DID of the issuer signing revoc_reg transaction to the Ledger
/// schema_seq_no: seq no of a schema transaction in Ledger
/// max_claim_num: maximum number of claims the new registry can process.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Revoc registry json
/// Unique number identifying the revocation registry in the wallet
///
/// #Errors
/// Common*
/// Wallet*
/// Anoncreds*
#[no_mangle]
pub extern "C" fn indy_issuer_create_and_store_revoc_reg(
    command_handle: i32,
    wallet_handle: i32,
    issuer_did: *const c_char,
    schema_seq_no: i32,
    max_claim_num: i32,
    cb: Option<
        extern "C" fn(xcommand_handle: i32,
                      err: ErrorCode,
                      revoc_reg_json: *const c_char),
    >,
) -> ErrorCode {
    check_useful_c_str!(issuer_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance().send(Command::Anoncreds(AnoncredsCommand::Issuer(
        IssuerCommand::CreateAndStoreRevocationRegistry(
            wallet_handle,
            issuer_did,
            schema_seq_no,
            max_claim_num,
            Box::new(move |result| {
                let (err, revoc_reg_json) = result_to_err_code_1!(result, String::new());
                let revoc_reg_json = CStringUtils::string_to_cstring(revoc_reg_json);
                cb(command_handle, err, revoc_reg_json.as_ptr())
            }),
        ),
    )));

    result_to_err_code!(result)
}

/// Signs a given claim for the given user by a given key (claim ef).
/// The corresponding claim definition and revocation registry must be already created
/// an stored into the wallet.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// claim_req_json: a claim request with a blinded secret
///     from the user (returned by prover_create_and_store_claim_req).
///     Also contains schema_seq_no and issuer_did
///     Example:
///     {
///      "blinded_ms" : <blinded_master_secret>,
///      "schema_seq_no" : <schema_seq_no>,
///      "issuer_did" : <issuer_did>
///     }
/// claim_json: a claim containing attribute values for each of requested attribute names.
///     Example:
///     {
///      "attr1" : ["value1", "value1_as_int"],
///      "attr2" : ["value2", "value2_as_int"]
///     }
/// user_revoc_index: index of a new user in the revocation registry (optional, pass -1 if user_revoc_index is absentee; default one is used if not provided)
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Revocation registry update json with a newly issued claim
/// Claim json containing issued claim, issuer_did, schema_seq_no, and revoc_reg_seq_no
/// used for issuance
///     {
///         "claim": <see claim_json above>,
///         "signature": <signature>,
///         "revoc_reg_seq_no", string,
///         "issuer_did", string,
///         "schema_seq_no", string,
///     }
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern "C" fn indy_issuer_create_claim(
    command_handle: i32,
    wallet_handle: i32,
    claim_req_json: *const c_char,
    claim_json: *const c_char,
    user_revoc_index: i32,
    cb: Option<
        extern "C" fn(xcommand_handle: i32,
                      err: ErrorCode,
                      revoc_reg_update_json: *const c_char, //TODO must be OPTIONAL
                      xclaim_json: *const c_char),
    >,
) -> ErrorCode {
    check_useful_c_str!(claim_req_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(claim_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    let user_revoc_index = if user_revoc_index != -1 {
        Some(user_revoc_index)
    } else {
        None
    };

    let result = CommandExecutor::instance().send(Command::Anoncreds(
        AnoncredsCommand::Issuer(IssuerCommand::CreateClaim(
            wallet_handle,
            claim_req_json,
            claim_json,
            user_revoc_index,
            Box::new(move |result| {
                let (err, revoc_reg_update_json, xclaim_json) =
                    result_to_err_code_2!(result, String::new(), String::new());
                let revoc_reg_update_json = CStringUtils::string_to_cstring(revoc_reg_update_json);
                let xclaim_json = CStringUtils::string_to_cstring(xclaim_json);
                cb(
                    command_handle,
                    err,
                    revoc_reg_update_json.as_ptr(),
                    xclaim_json.as_ptr(),
                )
            }),
        )),
    ));

    result_to_err_code!(result)
}

/// Revokes a user identified by a revoc_id in a given revoc-registry.
/// The corresponding claim definition and revocation registry must be already
/// created an stored into the wallet.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// issuer_did: a DID of the issuer signing claim_def transaction to the Ledger
/// schema_seq_no: seq no of a schema transaction in Ledger
/// user_revoc_index: index of the user in the revocation registry
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Revocation registry update json with a revoked claim
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern "C" fn indy_issuer_revoke_claim(
    command_handle: i32,
    wallet_handle: i32,
    issuer_did: *const c_char,
    schema_seq_no: i32,
    user_revoc_index: i32,
    cb: Option<
        extern "C" fn(xcommand_handle: i32,
                      err: ErrorCode,
                      revoc_reg_update_json: *const c_char),
    >,
) -> ErrorCode {
    check_useful_c_str!(issuer_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance().send(Command::Anoncreds(
        AnoncredsCommand::Issuer(IssuerCommand::RevokeClaim(
            wallet_handle,
            issuer_did,
            schema_seq_no,
            user_revoc_index,
            Box::new(move |result| {
                let (err, revoc_reg_update_json) = result_to_err_code_1!(result, String::new());
                let revoc_reg_update_json = CStringUtils::string_to_cstring(revoc_reg_update_json);
                cb(command_handle, err, revoc_reg_update_json.as_ptr())
            }),
        )),
    ));

    result_to_err_code!(result)
}

/// Stores a claim offer from the given issuer in a secure storage.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// claim_offer_json: claim offer as a json containing information about the issuer and a claim:
///        {
///            "issuer_did": string,
///            "schema_seq_no": string
///        }
///
/// #Returns
/// None.
///
/// #Errors
/// Common*
/// Wallet*
#[no_mangle]
pub extern "C" fn indy_prover_store_claim_offer(
    command_handle: i32,
    wallet_handle: i32,
    claim_offer_json: *const c_char,
    cb: Option<extern "C" fn(xcommand_handle: i32, err: ErrorCode)>,
) -> ErrorCode {
    check_useful_c_str!(claim_offer_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance().send(Command::Anoncreds(AnoncredsCommand::Prover(
        ProverCommand::StoreClaimOffer(
            wallet_handle,
            claim_offer_json,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            }),
        ),
    )));

    result_to_err_code!(result)
}

/// Gets all stored claim offers (see prover_store_claim_offer).
/// A filter can be specified to get claim offers for specific Issuer, claim_def or schema only.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// filter_json: optional filter to get claim offers for specific Issuer, claim_def or schema only only
///     Each of the filters is optional and can be combines
///        {
///            "issuer_did": string,
///            "schema_seq_no": string
///        }
///
/// #Returns
/// A json with a list of claim offers for the filter.
///        {
///            [{"issuer_did": string,
///            "schema_seq_no": string}]
///        }
///
/// #Errors
/// Common*
/// Wallet*
#[no_mangle]
pub extern "C" fn indy_prover_get_claim_offers(
    command_handle: i32,
    wallet_handle: i32,
    filter_json: *const c_char,
    cb: Option<
        extern "C" fn(xcommand_handle: i32,
                      err: ErrorCode,
                      claim_offers_json: *const c_char),
    >,
) -> ErrorCode {
    check_useful_c_str!(filter_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance().send(Command::Anoncreds(AnoncredsCommand::Prover(
        ProverCommand::GetClaimOffers(
            wallet_handle,
            filter_json,
            Box::new(move |result| {
                let (err, claim_offers_json) = result_to_err_code_1!(result, String::new());
                let claim_offers_json = CStringUtils::string_to_cstring(claim_offers_json);
                cb(command_handle, err, claim_offers_json.as_ptr())
            }),
        ),
    )));

    result_to_err_code!(result)
}


/// Creates a master secret with a given name and stores it in the wallet.
/// The name must be unique.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// master_secret_name: a new master secret name
///
/// #Returns
/// None.
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern "C" fn indy_prover_create_master_secret(
    command_handle: i32,
    wallet_handle: i32,
    master_secret_name: *const c_char,
    cb: Option<extern "C" fn(xcommand_handle: i32, err: ErrorCode)>,
) -> ErrorCode {
    check_useful_c_str!(master_secret_name, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance().send(Command::Anoncreds(AnoncredsCommand::Prover(
        ProverCommand::CreateMasterSecret(
            wallet_handle,
            master_secret_name,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            }),
        ),
    )));

    result_to_err_code!(result)
}


/// Creates a clam request json for the given claim offer and stores it in a secure wallet.
/// The claim offer contains the information about Issuer (DID, schema_seq_no),
/// and the schema (schema_seq_no).
/// The method gets public key and schema from the ledger, stores them in a wallet,
/// and creates a blinded master secret for a master secret identified by a provided name.
/// The master secret identified by the name must be already stored in the secure wallet (see prover_create_master_secret)
/// The blinded master secret is a part of the claim request.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// prover_did: a DID of the prover
/// claim_offer_json: claim offer as a json containing information about the issuer and a claim:
///        {
///            "issuer_did": string,
///            "schema_seq_no": string
///        }
/// claim_def_json: claim definition json associated with issuer_did and schema_seq_no in the claim_offer
/// master_secret_name: the name of the master secret stored in the wallet
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Claim request json.
///     {
///      "blinded_ms" : <blinded_master_secret>,
///      "schema_seq_no" : <schema_seq_no>,
///      "issuer_did" : <issuer_did>
///     }
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern "C" fn indy_prover_create_and_store_claim_req(
    command_handle: i32,
    wallet_handle: i32,
    prover_did: *const c_char,
    claim_offer_json: *const c_char,
    claim_def_json: *const c_char,
    master_secret_name: *const c_char,
    policy_address_name: *const c_char,
    cb: Option<
        extern "C" fn(xcommand_handle: i32,
                      err: ErrorCode,
                      claim_req_json: *const c_char),
    >,
) -> ErrorCode {
    check_useful_c_str!(prover_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(claim_offer_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(claim_def_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(master_secret_name, ErrorCode::CommonInvalidParam6);
    check_useful_opt_c_str!(policy_address_name, ErrorCode::CommonInvalidParam7);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam8);

    let result = CommandExecutor::instance().send(Command::Anoncreds(AnoncredsCommand::Prover(
        ProverCommand::CreateAndStoreClaimRequest(
            wallet_handle,
            prover_did,
            claim_offer_json,
            claim_def_json,
            master_secret_name,
            policy_address_name,
            Box::new(move |result| {
                let (err, claim_req_json) = result_to_err_code_1!(result, String::new());
                let claim_req_json = CStringUtils::string_to_cstring(claim_req_json);
                cb(command_handle, err, claim_req_json.as_ptr())
            }),
        ),
    )));

    result_to_err_code!(result)
}

/// Updates the claim by a master secret and stores in a secure wallet.
/// The claim contains the information about
/// schema_seq_no, issuer_did, revoc_reg_seq_no (see issuer_create_claim).
/// Seq_no is a sequence number of the corresponding transaction in the ledger.
/// The method loads a blinded secret for this key from the wallet,
/// updates the claim and stores it in a wallet.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// claims_json: claim json:
///     {
///         "claim": {attr1:[value, value_as_int]}
///         "signature": <signature>,
///         "schema_seq_no": string,
///         "revoc_reg_seq_no", string
///         "issuer_did", string
///     }
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// None
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern "C" fn indy_prover_store_claim(
    command_handle: i32,
    wallet_handle: i32,
    claims_json: *const c_char,
    cb: Option<extern "C" fn(xcommand_handle: i32, err: ErrorCode)>,
) -> ErrorCode {
    check_useful_c_str!(claims_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance().send(Command::Anoncreds(
        AnoncredsCommand::Prover(ProverCommand::StoreClaim(
            wallet_handle,
            claims_json,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            }),
        )),
    ));

    result_to_err_code!(result)
}


/// Gets human readable claims according to the filter.
/// If filter is NULL, then all claims are returned.
/// Claims can be filtered by Issuer, claim_def and/or Schema.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// filter_json: filter for claims
///     {
///         "issuer_did": string,
///         "schema_seq_no": string
///     }
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// claims json
///     [{
///         "claim_uuid": <string>,
///         "attrs": [{"attr_name" : "attr_value"}],
///         "schema_seq_no": string,
///         "issuer_did": string,
///         "revoc_reg_seq_no": string,
///     }]
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern "C" fn indy_prover_get_claims(
    command_handle: i32,
    wallet_handle: i32,
    filter_json: *const c_char,
    cb: Option<
        extern "C" fn(xcommand_handle: i32,
                      err: ErrorCode,
                      claims_json: *const c_char),
    >,
) -> ErrorCode {
    check_useful_c_str!(filter_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance().send(Command::Anoncreds(
        AnoncredsCommand::Prover(ProverCommand::GetClaims(
            wallet_handle,
            filter_json,
            Box::new(move |result| {
                let (err, claims_json) = result_to_err_code_1!(result, String::new());
                let claims_json = CStringUtils::string_to_cstring(claims_json);
                cb(command_handle, err, claims_json.as_ptr())
            }),
        )),
    ));

    result_to_err_code!(result)
}

/// Gets human readable claims matching the given proof request.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// proof_request_json: proof request json
///     {
///         "name": string,
///         "version": string,
///         "nonce": string,
///         "requested_attr1_uuid": <attr_info>,
///         "requested_attr2_uuid": <attr_info>,
///         "requested_attr3_uuid": <attr_info>,
///         "requested_predicate_1_uuid": <predicate_info>,
///         "requested_predicate_2_uuid": <predicate_info>,
///     }
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// json with claims for the given pool request.
/// Claim consists of uuid, human-readable attributes (key-value map), schema_seq_no, issuer_did and revoc_reg_seq_no.
///     {
///         "requested_attr1_uuid": [claim1, claim2],
///         "requested_attr2_uuid": [],
///         "requested_attr3_uuid": [claim3],
///         "requested_predicate_1_uuid": [claim1, claim3],
///         "requested_predicate_2_uuid": [claim2],
///     }, where claim is
///     {
///         "claim_uuid": <string>,
///         "attrs": [{"attr_name" : "attr_value"}],
///         "schema_seq_no": string,
///         "issuer_did": string,
///         "revoc_reg_seq_no": string,
///     }
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern "C" fn indy_prover_get_claims_for_proof_req(
    command_handle: i32,
    wallet_handle: i32,
    proof_request_json: *const c_char,
    cb: Option<
        extern "C" fn(xcommand_handle: i32,
                      err: ErrorCode,
                      claims_json: *const c_char),
    >,
) -> ErrorCode {
    check_useful_c_str!(proof_request_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance().send(Command::Anoncreds(AnoncredsCommand::Prover(
        ProverCommand::GetClaimsForProofReq(
            wallet_handle,
            proof_request_json,
            Box::new(move |result| {
                let (err, claims_json) = result_to_err_code_1!(result, String::new());
                let claims_json = CStringUtils::string_to_cstring(claims_json);
                cb(command_handle, err, claims_json.as_ptr())
            }),
        ),
    )));

    result_to_err_code!(result)
}

/// Creates a proof according to the given proof request
/// Either a corresponding claim with optionally revealed attributes or self-attested attribute must be provided
/// for each requested attribute (see indy_prover_get_claims_for_pool_req).
/// A proof request may request multiple claims from different schemas and different issuers.
/// All required schemas, public keys and revocation registries must be provided.
/// The proof request also contains nonce.
/// The proof contains either proof or self-attested attribute value for each requested attribute.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// proof_req_json: proof request json as come from the verifier
///     {
///         "nonce": string,
///         "requested_attr1_uuid": <attr_info>,
///         "requested_attr2_uuid": <attr_info>,
///         "requested_attr3_uuid": <attr_info>,
///         "requested_predicate_1_uuid": <predicate_info>,
///         "requested_predicate_2_uuid": <predicate_info>,
///     }
/// requested_claims_json: either a claim or self-attested attribute for each requested attribute
///     {
///         "requested_attr1_uuid": [claim1_uuid_in_wallet, true <reveal_attr>],
///         "requested_attr2_uuid": [self_attested_attribute],
///         "requested_attr3_uuid": [claim2_seq_no_in_wallet, false]
///         "requested_attr4_uuid": [claim2_seq_no_in_wallet, true]
///         "requested_predicate_1_uuid": [claim2_seq_no_in_wallet],
///         "requested_predicate_2_uuid": [claim3_seq_no_in_wallet],
///     }
/// schemas_jsons: all schema jsons participating in the proof request
///     {
///         "claim1_uuid_in_wallet": <schema1>,
///         "claim2_uuid_in_wallet": <schema2>,
///         "claim3_uuid_in_wallet": <schema3>,
///     }
///
/// master_secret_name: the name of the master secret stored in the wallet
/// claim_def_jsons: all claim definition jsons participating in the proof request
///     {
///         "claim1_uuid_in_wallet": <claim_def1>,
///         "claim2_uuid_in_wallet": <claim_def2>,
///         "claim3_uuid_in_wallet": <claim_def3>,
///     }
/// revoc_regs_jsons: all revocation registry jsons participating in the proof request
///     {
///         "claim1_uuid_in_wallet": <revoc_reg1>,
///         "claim2_uuid_in_wallet": <revoc_reg2>,
///         "claim3_uuid_in_wallet": <revoc_reg3>,
///     }
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Proof json
/// For each requested attribute either a proof (with optionally revealed attribute value) or
/// self-attested attribute value is provided.
/// Each proof is associated with a claim and corresponding schema_seq_no, issuer_did and revoc_reg_seq_no.
/// There ais also aggregated proof part common for all claim proofs.
///     {
///         "requested": {
///             "requested_attr1_id": [claim_proof1_uuid, revealed_attr1, revealed_attr1_as_int],
///             "requested_attr2_id": [self_attested_attribute],
///             "requested_attr3_id": [claim_proof2_uuid]
///             "requested_attr4_id": [claim_proof2_uuid, revealed_attr4, revealed_attr4_as_int],
///             "requested_predicate_1_uuid": [claim_proof2_uuid],
///             "requested_predicate_2_uuid": [claim_proof3_uuid],
///         }
///         "claim_proofs": {
///             "claim_proof1_uuid": [<claim_proof>, issuer_did, schema_seq_no, revoc_reg_seq_no],
///             "claim_proof2_uuid": [<claim_proof>, issuer_did, schema_seq_no, revoc_reg_seq_no],
///             "claim_proof3_uuid": [<claim_proof>, issuer_did, schema_seq_no, revoc_reg_seq_no]
///         },
///         "aggregated_proof": <aggregated_proof>
///     }
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern "C" fn indy_prover_create_proof(
    command_handle: i32,
    wallet_handle: i32,
    proof_req_json: *const c_char,
    requested_claims_json: *const c_char,
    schemas_json: *const c_char,
    master_secret_name: *const c_char,
    policy_address: *const c_char,
    agent_verkey: *const c_char,
    claim_defs_json: *const c_char,
    revoc_regs_json: *const c_char,
    cb: Option<
        extern "C" fn(xcommand_handle: i32,
                      err: ErrorCode,
                      proof_json: *const c_char),
    >,
) -> ErrorCode {
    check_useful_c_str!(proof_req_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(requested_claims_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(schemas_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(master_secret_name, ErrorCode::CommonInvalidParam6);
    check_useful_c_str!(policy_address, ErrorCode::CommonInvalidParam7);
    check_useful_c_str!(agent_verkey, ErrorCode::CommonInvalidParam8);
    check_useful_c_str!(claim_defs_json, ErrorCode::CommonInvalidParam9);
    check_useful_c_str!(revoc_regs_json, ErrorCode::CommonInvalidParam10);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam11);

    let result = CommandExecutor::instance().send(Command::Anoncreds(
        AnoncredsCommand::Prover(ProverCommand::CreateProof(
            wallet_handle,
            proof_req_json,
            requested_claims_json,
            schemas_json,
            master_secret_name,
            policy_address,
            agent_verkey,
            claim_defs_json,
            revoc_regs_json,
            Box::new(move |result| {
                let (err, proof_json) = result_to_err_code_1!(result, String::new());
                let proof_json = CStringUtils::string_to_cstring(proof_json);
                cb(command_handle, err, proof_json.as_ptr())
            }),
        )),
    ));

    result_to_err_code!(result)
}

/// Verifies a proof (of multiple claim).
/// All required schemas, public keys and revocation registries must be provided.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// proof_request_json: initial proof request as sent by the verifier
///     {
///         "nonce": string,
///         "requested_attr1_uuid": <attr_info>,
///         "requested_attr2_uuid": <attr_info>,
///         "requested_attr3_uuid": <attr_info>,
///         "requested_predicate_1_uuid": <predicate_info>,
///         "requested_predicate_2_uuid": <predicate_info>,
///     }
/// proof_json: proof json
/// For each requested attribute either a proof (with optionally revealed attribute value) or
/// self-attested attribute value is provided.
/// Each proof is associated with a claim and corresponding schema_seq_no, issuer_did and revoc_reg_seq_no.
/// There ais also aggregated proof part common for all claim proofs.
///     {
///         "requested": {
///             "requested_attr1_id": [claim_proof1_uuid, revealed_attr1, revealed_attr1_as_int],
///             "requested_attr2_id": [self_attested_attribute],
///             "requested_attr3_id": [claim_proof2_uuid]
///             "requested_attr4_id": [claim_proof2_uuid, revealed_attr4, revealed_attr4_as_int],
///             "requested_predicate_1_uuid": [claim_proof2_uuid],
///             "requested_predicate_2_uuid": [claim_proof3_uuid],
///         }
///         "claim_proofs": {
///             "claim_proof1_uuid": [<claim_proof>, issuer_did, schema_seq_no, revoc_reg_seq_no],
///             "claim_proof2_uuid": [<claim_proof>, issuer_did, schema_seq_no, revoc_reg_seq_no],
///             "claim_proof3_uuid": [<claim_proof>, issuer_did, schema_seq_no, revoc_reg_seq_no]
///         },
///         "aggregated_proof": <aggregated_proof>
///     }
/// schemas_jsons: all schema jsons participating in the proof
///         {
///             "claim_proof1_uuid": <schema>,
///             "claim_proof2_uuid": <schema>,
///             "claim_proof3_uuid": <schema>
///         }
/// claim_defs_jsons: all claim definition jsons participating in the proof
///         {
///             "claim_proof1_uuid": <claim_def>,
///             "claim_proof2_uuid": <claim_def>,
///             "claim_proof3_uuid": <claim_def>
///         }
/// revoc_regs_jsons: all revocation registry jsons participating in the proof
///         {
///             "claim_proof1_uuid": <revoc_reg>,
///             "claim_proof2_uuid": <revoc_reg>,
///             "claim_proof3_uuid": <revoc_reg>
///         }
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// valid: true - if signature is valid, false - otherwise
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern "C" fn indy_verifier_verify_proof(
    command_handle: i32,
    proof_request_json: *const c_char,
    proof_json: *const c_char,
    schemas_json: *const c_char,
    claim_defs_jsons: *const c_char,
    revoc_regs_json: *const c_char,
    accumulators: *const c_char,
    cb: Option<extern "C" fn(xcommand_handle: i32, err: ErrorCode, valid: bool)>,
) -> ErrorCode {
    check_useful_c_str!(proof_request_json, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(proof_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(schemas_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(claim_defs_jsons, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(revoc_regs_json, ErrorCode::CommonInvalidParam6);
    check_useful_opt_c_str!(accumulators, ErrorCode::CommonInvalidParam7);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam8);

    let result = CommandExecutor::instance().send(Command::Anoncreds(AnoncredsCommand::Verifier(
        VerifierCommand::VerifyProof(
            proof_request_json,
            proof_json,
            schemas_json,
            claim_defs_jsons,
            revoc_regs_json,
            accumulators,
            Box::new(move |result| {
                let (err, valid) = result_to_err_code_1!(result, false);
                cb(command_handle, err, valid)
            }),
        ),
    )));

    result_to_err_code!(result)
}
