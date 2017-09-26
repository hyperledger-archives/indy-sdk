from .libindy import do_call, create_cb

from typing import Optional
from ctypes import *

import logging


async def issuer_create_and_store_claim_def(wallet_handle: int,
                                            issuer_did: str,
                                            schema_json: str,
                                            signature_type: Optional[str],
                                            create_non_revoc: bool) -> str:
    """
    Create keys (both primary and revocation) for the given schema
    and signature type (currently only CL signature type is supported).
    Store the keys together with signature type and schema in a secure wallet as a claim definition.
    The claim definition in the wallet is identifying by a returned unique key.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param issuer_did: a DID of the issuer signing claim_def transaction to the Ledger
    :param schema_json: schema as a json
    :param signature_type: signature type (optional). Currently only 'CL' is supported.
    :param create_non_revoc: whether to request non-revocation claim.
    :return: claim definition json containing information about signature type, schema and issuer's public key.
            Unique number identifying the public key in the wallet
    """

    logger = logging.getLogger(__name__)
    logger.debug("issuer_create_and_store_claim_def: >>> wallet_handle: %r, issuer_did: %r, schema_json: %r,"
                 " signature_type: %r, create_non_revoc: %r",
                 wallet_handle,
                 issuer_did,
                 schema_json,
                 signature_type,
                 create_non_revoc)

    if not hasattr(issuer_create_and_store_claim_def, "cb"):
        logger.debug("issuer_create_and_store_claim_def: Creating callback")
        issuer_create_and_store_claim_def.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_issuer_did = c_char_p(issuer_did.encode('utf-8'))
    c_schema_json = c_char_p(schema_json.encode('utf-8'))
    c_signature_type = c_char_p(signature_type.encode('utf-8')) if signature_type is not None else None
    c_create_non_revoc = c_bool(create_non_revoc)

    claim_def_json = await do_call('indy_issuer_create_and_store_claim_def',
                                   c_wallet_handle,
                                   c_issuer_did,
                                   c_schema_json,
                                   c_signature_type,
                                   c_create_non_revoc,
                                   issuer_create_and_store_claim_def.cb)
    res = claim_def_json.decode()
    logger.debug("issuer_create_and_store_claim_def: <<< res: %r", res)
    return res


async def issuer_create_and_store_revoc_reg(wallet_handle: int,
                                            issuer_did: str,
                                            schema_seq_no: int,
                                            max_claim_num: int) -> str:
    """
    Create a new revocation registry for the given claim definition.
    Stores it in a secure wallet identifying by the returned key.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param issuer_did: a DID of the issuer signing revoc_reg transaction to the Ledger
    :param schema_seq_no: seq no of a schema transaction in Ledger
    :param max_claim_num: maximum number of claims the new registry can process.
    :return: Revoc registry json
    """

    logger = logging.getLogger(__name__)
    logger.debug("issuer_create_and_store_revoc_reg: >>> wallet_handle: %r, issuer_did: %r, schema_seq_no: %r,"
                 " max_claim_num: %r",
                 wallet_handle,
                 issuer_did,
                 schema_seq_no,
                 max_claim_num)

    if not hasattr(issuer_create_and_store_revoc_reg, "cb"):
        logger.debug("issuer_create_and_store_revoc_reg: Creating callback")
        issuer_create_and_store_revoc_reg.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_issuer_did = c_char_p(issuer_did.encode('utf-8'))
    c_schema_seq_no = c_int32(schema_seq_no)
    c_max_claim_num = c_int32(max_claim_num)

    revoc_reg_json = await do_call('indy_issuer_create_and_store_revoc_reg',
                                   c_wallet_handle,
                                   c_issuer_did,
                                   c_schema_seq_no,
                                   c_max_claim_num,
                                   issuer_create_and_store_revoc_reg.cb)
    res = revoc_reg_json.decode()
    logger.debug("issuer_create_and_store_revoc_reg: <<< res: %r", res)
    return res


async def issuer_create_claim(wallet_handle: int,
                              claim_req_json: str,
                              claim_json: str,
                              user_revoc_index: int) -> (str, str):
    """
    Signs a given claim for the given user by a given key (claim ef).
    The corresponding claim definition and revocation registry must be already created
    an stored into the wallet.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param claim_req_json: a claim request with a blinded secret
        from the user (returned by prover_create_and_store_claim_req).
        Also contains schema_seq_no and issuer_did
        Example:
        {
            "blinded_ms" : <blinded_master_secret>,
            "schema_seq_no" : <schema_seq_no>,
            "issuer_did" : <issuer_did>
        }
    :param claim_json: a claim containing attribute values for each of requested attribute names.
        Example:
        {
            "attr1" : ["value1", "value1_as_int"],
            "attr2" : ["value2", "value2_as_int"]
        }
    :param user_revoc_index: index of a new user in the revocation registry
     (optional, pass -1 if user_revoc_index is absentee; default one is used if not provided)
    :return: Revocation registry update json with a newly issued claim
        Claim json containing issued claim, issuer_did, and schema_seq_no
        used for issuance
        {
            "claim": <see claim_json above>,
            "signature": <signature>,
            "issuer_did", string,
            "schema_seq_no", string,
        }
    """

    logger = logging.getLogger(__name__)
    logger.debug("issuer_create_claim: >>> wallet_handle: %r, claim_req_json: %r, claim_json: %r,"
                 " user_revoc_index: %r",
                 wallet_handle,
                 claim_req_json,
                 claim_json,
                 user_revoc_index)

    if not hasattr(issuer_create_claim, "cb"):
        logger.debug("issuer_create_claim: Creating callback")
        issuer_create_claim.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_claim_req_json = c_char_p(claim_req_json.encode('utf-8'))
    c_claim_json = c_char_p(claim_json.encode('utf-8'))
    c_user_revoc_index = c_int32(user_revoc_index)

    (revoc_reg_update_json, claim_json) = await do_call('indy_issuer_create_claim',
                                                        c_wallet_handle,
                                                        c_claim_req_json,
                                                        c_claim_json,
                                                        c_user_revoc_index,
                                                        issuer_create_claim.cb)
    res = (revoc_reg_update_json.decode(), claim_json.decode())
    logger.debug("issuer_create_claim: <<< res: %r", res)
    return res


async def issuer_revoke_claim(wallet_handle: int,
                              issuer_did: str,
                              schema_seq_no: int,
                              user_revoc_index: int) -> str:
    """
    Revokes a user identified by a revoc_id in a given revoc-registry.
    The corresponding claim definition and revocation registry must be already
    created an stored into the wallet.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param issuer_did: a DID of the issuer signing claim_def transaction to the Ledger
    :param schema_seq_no: seq no of a schema transaction in Ledger
    :param user_revoc_index: index of the user in the revocation registry
    :return: Revocation registry update json with a revoked claim
    """

    logger = logging.getLogger(__name__)
    logger.debug("issuer_revoke_claim: >>> wallet_handle: %r, revoc_reg_seq_no: %r, user_revoc_index: %r",
                 wallet_handle,
                 issuer_did,
                 schema_seq_no,
                 user_revoc_index)

    if not hasattr(issuer_revoke_claim, "cb"):
        logger.debug("issuer_revoke_claim: Creating callback")
        issuer_revoke_claim.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_issuer_did = c_char_p(issuer_did.encode('utf-8'))
    c_schema_seq_no = c_int32(schema_seq_no)
    c_user_revoc_index = c_int32(user_revoc_index)

    revoc_reg_update_json = await do_call('indy_issuer_revoke_claim',
                                          c_wallet_handle,
                                          c_issuer_did,
                                          c_schema_seq_no,
                                          c_user_revoc_index,
                                          issuer_revoke_claim.cb)
    res = revoc_reg_update_json.decode()
    logger.debug("issuer_revoke_claim: <<< res: %r", res)
    return res


async def prover_store_claim_offer(wallet_handle: int,
                                   claim_offer_json: str) -> None:
    """
    Stores a claim offer from the given issuer in a secure storage.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param claim_offer_json: claim offer as a json containing information about the issuer and a claim:
        {
            "issuer_did": string,
            "schema_seq_no": string
        }
    :return: None.
    """

    logger = logging.getLogger(__name__)
    logger.debug("prover_store_claim_offer: >>> wallet_handle: %r, claim_offer_json: %r",
                 wallet_handle,
                 claim_offer_json)

    if not hasattr(prover_store_claim_offer, "cb"):
        logger.debug("prover_store_claim_offer: Creating callback")
        prover_store_claim_offer.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_wallet_handle = c_int32(wallet_handle)
    c_claim_offer_json = c_char_p(claim_offer_json.encode('utf-8'))

    res = await do_call('indy_prover_store_claim_offer',
                        c_wallet_handle,
                        c_claim_offer_json,
                        prover_store_claim_offer.cb)

    logger.debug("prover_store_claim_offer: <<< res: %r", res)
    return res


async def prover_get_claim_offers(wallet_handle: int,
                                  filter_json: str) -> str:
    """
    Gets all stored claim offers (see prover_store_claim_offer).
    A filter can be specified to get claim offers for specific Issuer, claim_def or schema only.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param filter_json: optional filter to get claim offers for specific Issuer, claim_def or schema only only
        Each of the filters is optional and can be combines
            {
                "issuer_did": string,
                "schema_seq_no": string
            }
    :return: A json with a list of claim offers for the filter.
        {
            [{"issuer_did": string,
            "schema_seq_no": string}]
        }
    """

    logger = logging.getLogger(__name__)
    logger.debug("prover_store_claim_offer: >>> wallet_handle: %r, filter_json: %r",
                 wallet_handle,
                 filter_json)

    if not hasattr(prover_get_claim_offers, "cb"):
        logger.debug("prover_get_claim_offers: Creating callback")
        prover_get_claim_offers.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_filter_json = c_char_p(filter_json.encode('utf-8'))

    claim_offers_json = await do_call('indy_prover_get_claim_offers',
                                      c_wallet_handle,
                                      c_filter_json,
                                      prover_get_claim_offers.cb)

    res = claim_offers_json.decode()
    logger.debug("prover_get_claim_offers: <<< res: %r", res)
    return res


async def prover_create_master_secret(wallet_handle: int,
                                      master_secret_name: str) -> None:
    """
    Creates a master secret with a given name and stores it in the wallet.
    The name must be unique.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param master_secret_name: a new master secret name
    :return: None.
    """

    logger = logging.getLogger(__name__)
    logger.debug("prover_create_master_secret: >>> wallet_handle: %r, master_secret_name: %r",
                 wallet_handle,
                 master_secret_name)

    if not hasattr(prover_create_master_secret, "cb"):
        logger.debug("prover_create_master_secret: Creating callback")
        prover_create_master_secret.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_wallet_handle = c_int32(wallet_handle)
    c_master_secret_name = c_char_p(master_secret_name.encode('utf-8'))

    res = await do_call('indy_prover_create_master_secret',
                        c_wallet_handle,
                        c_master_secret_name,
                        prover_create_master_secret.cb)

    logger.debug("prover_create_master_secret: <<< res: %r", res)
    return res


async def prover_create_and_store_claim_req(wallet_handle: int,
                                            prover_did: str,
                                            claim_offer_json: str,
                                            claim_def_json: str,
                                            master_secret_name: str) -> str:
    """
    Creates a clam request json for the given claim offer and stores it in a secure wallet.
    The claim offer contains the information about Issuer (DID, schema_seq_no),
    and the schema (schema_seq_no).
    The method gets public key and schema from the ledger, stores them in a wallet,
    and creates a blinded master secret for a master secret identified by a provided name.
    The master secret identified by the name must be already stored in the secure wallet (see prover_create_master_secret)
    The blinded master secret is a part of the claim request.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param prover_did: a DID of the prover
    :param claim_offer_json: claim offer as a json containing information about the issuer and a claim:
        {
            "issuer_did": string,
            "schema_seq_no": string
        }
    :param claim_def_json: claim definition json associated with issuer_did and schema_seq_no in the claim_offer
    :param master_secret_name: the name of the master secret stored in the wallet
    :return: Claim request json.
        {
            "blinded_ms" : <blinded_master_secret>,
            "schema_seq_no" : <schema_seq_no>,
            "issuer_did" : <issuer_did>
        }
    """

    logger = logging.getLogger(__name__)
    logger.debug("prover_create_and_store_claim_req: >>> wallet_handle: %r, prover_did: %r, claim_offer_json: %r,"
                 " claim_def_json: %r, master_secret_name: %r",
                 wallet_handle,
                 prover_did,
                 claim_offer_json,
                 claim_def_json,
                 master_secret_name)

    if not hasattr(prover_create_and_store_claim_req, "cb"):
        logger.debug("prover_create_and_store_claim_req: Creating callback")
        prover_create_and_store_claim_req.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_prover_did = c_char_p(prover_did.encode('utf-8'))
    c_claim_offer_json = c_char_p(claim_offer_json.encode('utf-8'))
    c_claim_def_json = c_char_p(claim_def_json.encode('utf-8'))
    c_master_secret_name = c_char_p(master_secret_name.encode('utf-8'))

    claim_req_json = await do_call('indy_prover_create_and_store_claim_req',
                                   c_wallet_handle,
                                   c_prover_did,
                                   c_claim_offer_json,
                                   c_claim_def_json,
                                   c_master_secret_name,
                                   prover_create_and_store_claim_req.cb)

    res = claim_req_json.decode()
    logger.debug("prover_create_and_store_claim_req: <<< res: %r", res)
    return res


async def prover_store_claim(wallet_handle: int,
                             claims_json: str) -> None:
    """
    Updates the claim by a master secret and stores in a secure wallet.
    The claim contains the information about
    schema_seq_no, issuer_did, revoc_reg_seq_no (see issuer_create_claim).
    Seq_no is a sequence number of the corresponding transaction in the ledger.
    The method loads a blinded secret for this key from the wallet,
    updates the claim and stores it in a wallet.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param claims_json: claim json:
        {
            "claim": {attr1:[value, value_as_int]}
            "signature": <signature>,
            "schema_seq_no": string,
            "revoc_reg_seq_no", string
            "issuer_did", string
        }
    :return: None.
    """

    logger = logging.getLogger(__name__)
    logger.debug("prover_store_claim: >>> wallet_handle: %r, claims_json: %r",
                 wallet_handle,
                 claims_json)

    if not hasattr(prover_store_claim, "cb"):
        logger.debug("prover_store_claim: Creating callback")
        prover_store_claim.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_wallet_handle = c_int32(wallet_handle)
    c_claims_json = c_char_p(claims_json.encode('utf-8'))

    res = await do_call('indy_prover_store_claim',
                        c_wallet_handle,
                        c_claims_json,
                        prover_store_claim.cb)

    logger.debug("prover_store_claim: <<< res: %r", res)
    return res


async def prover_get_claims(wallet_handle: int,
                            filter_json: str) -> str:
    """
    Gets human readable claims according to the filter.
    If filter is NULL, then all claims are returned.
    Claims can be filtered by Issuer, claim_def and/or Schema.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param filter_json: filter for claims
        {
            "issuer_did": string,
            "schema_seq_no": string
        }
    :return: claims json
        [{
            "claim_uuid": <string>,
            "attrs": [{"attr_name" : "attr_value"}],
            "schema_seq_no": string,
            "issuer_did": string,
            "revoc_reg_seq_no": string,
        }]
    """

    logger = logging.getLogger(__name__)
    logger.debug("prover_get_claims: >>> wallet_handle: %r, filter_json: %r",
                 wallet_handle,
                 filter_json)

    if not hasattr(prover_get_claims, "cb"):
        logger.debug("prover_get_claims: Creating callback")
        prover_get_claims.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_filter_json = c_char_p(filter_json.encode('utf-8'))

    claims_json = await do_call('indy_prover_get_claims',
                                c_wallet_handle,
                                c_filter_json,
                                prover_get_claims.cb)

    res = claims_json.decode()
    logger.debug("prover_get_claims: <<< res: %r", res)
    return res


async def prover_get_claims_for_proof_req(wallet_handle: int,
                                          proof_request_json: str) -> str:
    """
    Gets human readable claims matching the given proof request.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param proof_request_json: proof request json
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
    :return: json with claims for the given pool request.
        Claim consists of uuid, human-readable attributes (key-value map), schema_seq_no, issuer_did and revoc_reg_seq_no.
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
    """

    logger = logging.getLogger(__name__)
    logger.debug("prover_get_claims_for_proof_req: >>> wallet_handle: %r, proof_request_json: %r",
                 wallet_handle,
                 proof_request_json)

    if not hasattr(prover_get_claims, "cb"):
        logger.debug("prover_get_claims_for_proof_req: Creating callback")
        prover_get_claims_for_proof_req.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_proof_request_json = c_char_p(proof_request_json.encode('utf-8'))

    claims_json = await do_call('indy_prover_get_claims_for_proof_req',
                                c_wallet_handle,
                                c_proof_request_json,
                                prover_get_claims_for_proof_req.cb)

    res = claims_json.decode()
    logger.debug("prover_get_claims_for_proof_req: <<< res: %r", res)
    return res


async def prover_create_proof(wallet_handle: int,
                              proof_req_json: str,
                              requested_claims_json: str,
                              schemas_json: str,
                              master_secret_name: str,
                              claim_defs_json: str,
                              revoc_regs_json: str) -> str:
    """
    Creates a proof according to the given proof request
    Either a corresponding claim with optionally revealed attributes or self-attested attribute must be provided
    for each requested attribute (see indy_prover_get_claims_for_pool_req).
    A proof request may request multiple claims from different schemas and different issuers.
    All required schemas, public keys and revocation registries must be provided.
    The proof request also contains nonce.
    The proof contains either proof or self-attested attribute value for each requested attribute.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param proof_req_json: proof request json as come from the verifier
        {
            "nonce": string,
            "requested_attr1_uuid": <attr_info>,
            "requested_attr2_uuid": <attr_info>,
            "requested_attr3_uuid": <attr_info>,
            "requested_predicate_1_uuid": <predicate_info>,
            "requested_predicate_2_uuid": <predicate_info>,
        }
    :param requested_claims_json: either a claim or self-attested attribute for each requested attribute
        {
            "requested_attr1_uuid": [claim1_uuid_in_wallet, true <reveal_attr>],
            "requested_attr2_uuid": [self_attested_attribute],
            "requested_attr3_uuid": [claim2_seq_no_in_wallet, false]
            "requested_attr4_uuid": [claim2_seq_no_in_wallet, true]
            "requested_predicate_1_uuid": [claim2_seq_no_in_wallet],
            "requested_predicate_2_uuid": [claim3_seq_no_in_wallet],
        }
    :param schemas_json: all schema jsons participating in the proof request
        {
            "claim1_uuid_in_wallet": <schema1>,
            "claim2_uuid_in_wallet": <schema2>,
            "claim3_uuid_in_wallet": <schema3>,
        }
    :param master_secret_name: the name of the master secret stored in the wallet

    :param claim_defs_json: all claim definition jsons participating in the proof request
        {
            "claim1_uuid_in_wallet": <claim_def1>,
            "claim2_uuid_in_wallet": <claim_def2>,
            "claim3_uuid_in_wallet": <claim_def3>,
        }
    :param revoc_regs_json: all revocation registry jsons participating in the proof request
        {
            "claim1_uuid_in_wallet": <revoc_reg1>,
            "claim2_uuid_in_wallet": <revoc_reg2>,
            "claim3_uuid_in_wallet": <revoc_reg3>,
        }
    :return: Proof json
        For each requested attribute either a proof (with optionally revealed attribute value) or
        self-attested attribute value is provided.
        Each proof is associated with a claim and corresponding schema_seq_no, issuer_did and revoc_reg_seq_no.
        There ais also aggregated proof part common for all claim proofs.
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
    """

    logger = logging.getLogger(__name__)
    logger.debug("prover_create_proof: >>> wallet_handle: %r, proof_req_json: %r,"
                 " requested_claims_json: %r, schemas_json: %r, master_secret_name: %r,"
                 " claim_defs_json: %r, revoc_regs_json: %r",
                 wallet_handle,
                 proof_req_json,
                 requested_claims_json,
                 schemas_json,
                 master_secret_name,
                 claim_defs_json,
                 revoc_regs_json)

    if not hasattr(prover_create_proof, "cb"):
        logger.debug("prover_create_proof: Creating callback")
        prover_create_proof.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_proof_req_json = c_char_p(proof_req_json.encode('utf-8'))
    c_requested_claims_json = c_char_p(requested_claims_json.encode('utf-8'))
    c_schemas_json = c_char_p(schemas_json.encode('utf-8'))
    c_master_secret_name = c_char_p(master_secret_name.encode('utf-8'))
    c_claim_defs_json = c_char_p(claim_defs_json.encode('utf-8'))
    c_revoc_regs_json = c_char_p(revoc_regs_json.encode('utf-8'))

    proof_json = await do_call('indy_prover_create_proof',
                               c_wallet_handle,
                               c_proof_req_json,
                               c_requested_claims_json,
                               c_schemas_json,
                               c_master_secret_name,
                               c_claim_defs_json,
                               c_revoc_regs_json,
                               prover_create_proof.cb)

    res = proof_json.decode()
    logger.debug("prover_create_proof: <<< res: %r", res)
    return res


async def verifier_verify_proof(proof_request_json: str,
                                proof_json: str,
                                schemas_json: str,
                                claim_defs_jsons: str,
                                revoc_regs_json: str) -> bool:
    """
    Verifies a proof (of multiple claim).
    All required schemas, public keys and revocation registries must be provided.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param proof_request_json: initial proof request as sent by the verifier
        {
            "nonce": string,
            "requested_attr1_uuid": <attr_info>,
            "requested_attr2_uuid": <attr_info>,
            "requested_attr3_uuid": <attr_info>,
            "requested_predicate_1_uuid": <predicate_info>,
            "requested_predicate_2_uuid": <predicate_info>,
        }
    :param proof_json: proof json
        For each requested attribute either a proof (with optionally revealed attribute value) or
        self-attested attribute value is provided.
        Each proof is associated with a claim and corresponding schema_seq_no, issuer_did and revoc_reg_seq_no.
        There ais also aggregated proof part common for all claim proofs.
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
    :param schemas_json: all schema jsons participating in the proof
        {
            "claim_proof1_uuid": <schema>,
            "claim_proof2_uuid": <schema>,
            "claim_proof3_uuid": <schema>
        }
    :param claim_defs_jsons: all claim definition jsons participating in the proof
        {
            "claim_proof1_uuid": <claim_def>,
            "claim_proof2_uuid": <claim_def>,
            "claim_proof3_uuid": <claim_def>
        }
    :param revoc_regs_json: all revocation registry jsons participating in the proof
        {
            "claim_proof1_uuid": <revoc_reg>,
            "claim_proof2_uuid": <revoc_reg>,
            "claim_proof3_uuid": <revoc_reg>
        }
    :return: valid: true - if signature is valid, false - otherwise
    """

    logger = logging.getLogger(__name__)
    logger.debug("verifier_verify_proof: >>> proof_request_json: %r,"
                 " proof_json: %r, schemas_json: %r, claim_defs_jsons: %r, revoc_regs_json: %r",
                 proof_request_json,
                 proof_json,
                 schemas_json,
                 claim_defs_jsons,
                 revoc_regs_json)

    if not hasattr(verifier_verify_proof, "cb"):
        logger.debug("verifier_verify_proof: Creating callback")
        verifier_verify_proof.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_bool))

    c_proof_request_json = c_char_p(proof_request_json.encode('utf-8'))
    c_proof_json = c_char_p(proof_json.encode('utf-8'))
    c_schemas_json = c_char_p(schemas_json.encode('utf-8'))
    c_claim_defs_jsons = c_char_p(claim_defs_jsons.encode('utf-8'))
    c_revoc_regs_json = c_char_p(revoc_regs_json.encode('utf-8'))

    res = await do_call('indy_verifier_verify_proof',
                        c_proof_request_json,
                        c_proof_json,
                        c_schemas_json,
                        c_claim_defs_jsons,
                        c_revoc_regs_json,
                        verifier_verify_proof.cb)

    logger.debug("verifier_verify_proof: <<< res: %r", res)
    return res
