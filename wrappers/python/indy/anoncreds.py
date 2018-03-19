from .libindy import do_call, create_cb

from typing import Optional
from ctypes import *

import logging


async def issuer_create_schema(issuer_did: str,
                               name: str,
                               version: str,
                               attr_names: str) -> (str, str):
    """
    Create credential schema.

    :param issuer_did: a DID of the issuer signing credential_def transaction to the Ledger
    :param name: human-readable name of schema.
    :param version: version of schema.
    :param attr_names: list of attributes schema contains.
    :return: schema id and schema json
    """

    logger = logging.getLogger(__name__)
    logger.debug("issuer_create_schema: >>> issuer_did: %r, name: %r, version: %r, attr_names: %r",
                 issuer_did,
                 name,
                 version,
                 attr_names)

    if not hasattr(issuer_create_schema, "cb"):
        logger.debug("issuer_create_schema: Creating callback")
        issuer_create_schema.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p, c_char_p))

    c_issuer_did = c_char_p(issuer_did.encode('utf-8'))
    c_name = c_char_p(name.encode('utf-8'))
    c_version = c_char_p(version.encode('utf-8'))
    c_attr_names = c_char_p(attr_names.encode('utf-8'))

    (schema_id, schema_json) = await do_call('indy_issuer_create_schema',
                                             c_issuer_did,
                                             c_name,
                                             c_version,
                                             c_attr_names,
                                             issuer_create_schema.cb)

    res = (schema_id.decode(), schema_json.decode())
    logger.debug("issuer_create_schema: <<< res: %r", res)
    return res


async def issuer_create_and_store_credential_def(wallet_handle: int,
                                                 issuer_did: str,
                                                 schema_json: str,
                                                 tag: str,
                                                 type_: Optional[str],
                                                 config_json: str) -> (str, str):
    """
    Create keys (both primary and revocation) for the given schema
    and signature type (currently only CL signature type is supported).
    Store the keys together with signature type and schema in a secure wallet as a credential definition.
    The credential definition in the wallet is identifying by a returned unique key.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param issuer_did: a DID of the issuer signing credential_def transaction to the Ledger
    :param schema_json: schema as a json
    :param tag: schema as a json
    :param type_: (optional) signature type. Currently only 'CL' is supported.
    :param config_json: { "support_revocation": boolean }.
    :return: credential definition json containing information about signature type, schema and issuer's public key.
            Unique number identifying the public key in the wallet
    """

    logger = logging.getLogger(__name__)
    logger.debug("issuer_create_and_store_credential_def: >>> wallet_handle: %r, issuer_did: %r, schema_json: %r,"
                 " tag: %r, type_: %r, config_json: %r",
                 wallet_handle,
                 issuer_did,
                 schema_json,
                 tag,
                 type_,
                 config_json)

    if not hasattr(issuer_create_and_store_credential_def, "cb"):
        logger.debug("issuer_create_and_store_credential_def: Creating callback")
        issuer_create_and_store_credential_def.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_issuer_did = c_char_p(issuer_did.encode('utf-8'))
    c_schema_json = c_char_p(schema_json.encode('utf-8'))
    c_tag = c_char_p(tag.encode('utf-8'))
    c_type = c_char_p(type_.encode('utf-8')) if type_ is not None else None
    c_config_json = c_char_p(config_json.encode('utf-8'))

    (credential_def_id, credential_def_json) = await do_call('indy_issuer_create_and_store_credential_def',
                                                             c_wallet_handle,
                                                             c_issuer_did,
                                                             c_schema_json,
                                                             c_tag,
                                                             c_type,
                                                             c_config_json,
                                                             issuer_create_and_store_credential_def.cb)

    res = (credential_def_id.decode(), credential_def_json.decode())
    logger.debug("issuer_create_and_store_credential_def: <<< res: %r", res)
    return res


async def issuer_create_and_store_revoc_reg(wallet_handle: int,
                                            issuer_did: str,
                                            type_: Optional[str],
                                            tag: str,
                                            cred_def_id: str,
                                            config_json: str,
                                            tails_writer_type: Optional[str],
                                            tails_writer_config: str) -> (str, str, str):
    """
    Create a new revocation registry for the given credential definition.
    Stores it in a secure wallet identifying by the returned key.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param issuer_did: a DID of the issuer signing revoc_reg transaction to the Ledger
    :param type_: (optional) registry type. Currently only 'CL_ACCUM' is supported.
    :param tag:
    :param cred_def_id: id of stored in ledger credential definition
    :param config_json: {
        "issuance_type": (optional) type of issuance. Currently supported:
                1) ISSUANCE_BY_DEFAULT: all indices are assumed to be issued and initial accumulator is calculated over
                                     all indices; Revocation Registry is updated only during revocation.
             2) ISSUANCE_ON_DEMAND: nothing is issued initially accumulator is 1 (used by default);
        "max_cred_num": maximum number of credentials the new registry can process.
    }
    :param tails_writer_type:
    :param tails_writer_config:
    :return: Revocation registry id, definition json and entry json
    """

    logger = logging.getLogger(__name__)
    logger.debug("issuer_create_and_store_revoc_reg: >>> wallet_handle: %r, issuer_did: %r, type_: %r,"
                 " tag: %r, cred_def_id: %r, config_json: %r, tails_writer_type: %r, tails_writer_config: %r",
                 wallet_handle,
                 issuer_did,
                 type_,
                 tag,
                 cred_def_id,
                 config_json,
                 tails_writer_type,
                 tails_writer_config)

    if not hasattr(issuer_create_and_store_revoc_reg, "cb"):
        logger.debug("issuer_create_and_store_revoc_reg: Creating callback")
        issuer_create_and_store_revoc_reg.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p, c_char_p, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_issuer_did = c_char_p(issuer_did.encode('utf-8'))
    c_type = c_char_p(type_.encode('utf-8')) if type_ is not None else None
    c_tag = c_char_p(tag.encode('utf-8'))
    c_cred_def_id = c_char_p(cred_def_id.encode('utf-8'))
    c_config_json = c_char_p(config_json.encode('utf-8'))
    c_tails_writer_type = c_char_p(tails_writer_type.encode('utf-8')) if tails_writer_type is not None else None
    c_tails_writer_config = c_char_p(tails_writer_config.encode('utf-8'))

    (rev_reg_id, rev_reg_def_json, rev_reg_entry_json) = await do_call('indy_issuer_create_and_store_revoc_reg',
                                                                       c_wallet_handle,
                                                                       c_issuer_did,
                                                                       c_type,
                                                                       c_tag,
                                                                       c_cred_def_id,
                                                                       c_config_json,
                                                                       c_tails_writer_type,
                                                                       c_tails_writer_config,
                                                                       issuer_create_and_store_revoc_reg.cb)
    res = (rev_reg_id.decode(), rev_reg_def_json.decode(), rev_reg_entry_json.decode())
    logger.debug("issuer_create_and_store_revoc_reg: <<< res: %r", res)
    return res


async def issuer_create_credential_offer(wallet_handle: int,
                                         cred_def_id: str,
                                         issuer_did: str,
                                         prover_did: str) -> str:
    """
    Create credential offer in Wallet.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param cred_def_id: id of stored in ledger credential definition
    :param issuer_did: a DID of the issuer of credential
    :param prover_did: a DID of the target use
    :return:
    credential offer json: {
        "cred_def_id": string,
        "issuer_did" : string,
        "nonce": string,
        "key_correctness_proof" : <key_correctness_proof>
    }
    """

    logger = logging.getLogger(__name__)
    logger.debug("issuer_create_credential_offer: >>> wallet_handle: %r, cred_def_id: %r, issuer_did: %r,"
                 " prover_did: %r",
                 wallet_handle,
                 cred_def_id,
                 issuer_did,
                 prover_did)

    if not hasattr(issuer_create_credential_offer, "cb"):
        logger.debug("issuer_create_credential_offer: Creating callback")
        issuer_create_credential_offer.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_cred_def_id = c_char_p(cred_def_id.encode('utf-8'))
    c_issuer_did = c_char_p(issuer_did.encode('utf-8'))
    c_prover_did = c_char_p(prover_did.encode('utf-8'))

    credential_offer_json = await do_call('indy_issuer_create_credential_offer',
                                          c_wallet_handle,
                                          c_cred_def_id,
                                          c_issuer_did,
                                          c_prover_did,
                                          issuer_create_credential_offer.cb)

    res = credential_offer_json.decode()
    logger.debug("issuer_create_credential_offer: <<< res: %r", res)
    return res


async def issuer_create_credential(wallet_handle: int,
                                   credential_req_json: str,
                                   credential_values_json: str,
                                   rev_reg_id: Optional[str],
                                   tails_reader_handle: Optional[int],
                                   user_revoc_index: Optional[int]) -> (str, str):
    """
    Signs a given credential for the given user by a given key (credential ef).
    The corresponding credential definition and revocation registry must be already created
    an stored into the wallet.

    :param wallet_handle: wallet handle (created by open_wallet).
    :param credential_req_json: a credential request with a blinded secret
        from the user (returned by prover_create_and_store_credential_req).
        Example:
        {
            "blinded_ms" : <blinded_master_secret>,
            "cred_def_id" : string,
            "issuer_did" : string
            "prover_did" : string,
            "blinded_ms_correctness_proof": <blinded_ms_correctness_proof>,
            "nonce": string
        }
    :param credential_values_json: a credential containing attribute values for each of requested attribute names.
        Example:
        {
          "attr1" : {"raw": "value1", "encoded": "value1_as_int" },
          "attr2" : {"raw": "value1", "encoded": "value1_as_int" }
        }
    :param rev_reg_id: (Optional) id of stored in ledger revocation registry definition
    :param tails_reader_handle: (Optional)
    :param user_revoc_index: (Optional)  index of a new user in the revocation registry
     (optional, pass -1 if user_revoc_index is absentee; default one is used if not provided)
    :return: Revocation registry update json with a newly issued credential credential json
        {
            "values": <see credential_values_json above>,
            "signature": <signature>,
            "issuer_did": string,
            "cred_def_id": string,
            "rev_reg_id", Optional<string>,
            "signature_correctness_proof": <signature_correctness_proof>
        }
    """

    logger = logging.getLogger(__name__)
    logger.debug("issuer_create_credential: >>> wallet_handle: %r, credential_req_json: %r, credential_values_json: %r,"
                 " rev_reg_id: %r, tails_reader_handle: %r, user_revoc_index: %r",
                 wallet_handle,
                 credential_req_json,
                 credential_values_json,
                 rev_reg_id,
                 tails_reader_handle,
                 user_revoc_index)

    if not hasattr(issuer_create_credential, "cb"):
        logger.debug("issuer_create_credential: Creating callback")
        issuer_create_credential.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_credential_req_json = c_char_p(credential_req_json.encode('utf-8'))
    c_credential_json = c_char_p(credential_values_json.encode('utf-8'))
    c_rev_reg_id = c_char_p(rev_reg_id.encode('utf-8')) if rev_reg_id is not None else None
    c_tails_reader_handle = c_int32(tails_reader_handle) if tails_reader_handle else -1
    c_user_revoc_index = c_int32(user_revoc_index) if user_revoc_index else -1

    (revoc_reg_delta_json, credential_json) = await do_call('indy_issuer_create_credential',
                                                            c_wallet_handle,
                                                            c_credential_req_json,
                                                            c_credential_json,
                                                            c_rev_reg_id,
                                                            c_tails_reader_handle,
                                                            c_user_revoc_index,
                                                            issuer_create_credential.cb)
    credential_json = credential_json.decode()
    revoc_reg_delta_json = revoc_reg_delta_json.decode() if revoc_reg_delta_json else None
    res = (revoc_reg_delta_json, credential_json)

    logger.debug("issuer_create_credential: <<< res: %r", res)
    return res


async def issuer_revoke_credential(wallet_handle: int,
                                   tails_reader_handle: int,
                                   rev_reg_id: str,
                                   user_revoc_index: int) -> str:
    """
    Revokes a user identified by a revoc_id in a given revoc-registry.
    The corresponding credential definition and revocation registry must be already
    created an stored into the wallet.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param tails_reader_handle:
    :param rev_reg_id: id of revocation registry stored in wallet
    :param user_revoc_index: index of the user in the revocation registry
    :return: Revocation registry update json with a revoked credential
    """

    logger = logging.getLogger(__name__)
    logger.debug(
        "issuer_revoke_credential: >>> wallet_handle: %r, tails_reader_handle: %r, rev_reg_id: %r",
        wallet_handle,
        tails_reader_handle,
        rev_reg_id,
        user_revoc_index)

    if not hasattr(issuer_revoke_credential, "cb"):
        logger.debug("issuer_revoke_credential: Creating callback")
        issuer_revoke_credential.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_tails_reader_handle = c_int32(tails_reader_handle)
    c_rev_reg_id = c_char_p(rev_reg_id.encode('utf-8'))
    c_user_revoc_index = c_int32(user_revoc_index)

    revoc_reg_delta_json = await do_call('indy_issuer_revoke_credential',
                                         c_wallet_handle,
                                         c_tails_reader_handle,
                                         c_rev_reg_id,
                                         c_user_revoc_index,
                                         issuer_revoke_credential.cb)
    res = revoc_reg_delta_json.decode()
    logger.debug("issuer_revoke_credential: <<< res: %r", res)
    return res


async def issuer_recovery_credential(wallet_handle: int,
                                     tails_reader_handle: int,
                                     rev_reg_id: str,
                                     user_revoc_index: int) -> str:
    """
    Recover a user identified by a user_revoc_index in a given revoc-registry.
    The corresponding credential definition and revocation registry must be already
    created an stored into the wallet.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param tails_reader_handle:
    :param rev_reg_id: id of revocation registry stored in wallet
    :param user_revoc_index: index of the user in the revocation registry
    :return: Revocation registry update json with a revoked credential
    """

    logger = logging.getLogger(__name__)
    logger.debug(
        "issuer_recovery_credential: >>> wallet_handle: %r, tails_reader_handle: %r, rev_reg_id: %r",
        wallet_handle,
        tails_reader_handle,
        rev_reg_id,
        user_revoc_index)

    if not hasattr(issuer_recovery_credential, "cb"):
        logger.debug("issuer_recovery_credential: Creating callback")
        issuer_recovery_credential.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_tails_reader_handle = c_int32(tails_reader_handle)
    c_rev_reg_id = c_char_p(rev_reg_id.encode('utf-8'))
    c_user_revoc_index = c_int32(user_revoc_index)

    revoc_reg_delta_json = await do_call('indy_issuer_recover_credential',
                                         c_wallet_handle,
                                         c_tails_reader_handle,
                                         c_rev_reg_id,
                                         c_user_revoc_index,
                                         issuer_recovery_credential.cb)
    res = revoc_reg_delta_json.decode()
    logger.debug("issuer_recovery_credential: <<< res: %r", res)
    return res


async def prover_store_credential_offer(wallet_handle: int,
                                        credential_offer_json: str) -> None:
    """
    Stores a credential offer from the given issuer in a secure storage.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param credential_offer_json: credential offer as a json containing information about the issuer and a credential:
       {
           "cred_def_id": string,
           "rev_reg_id" : Optional<string>,
           "nonce": string,
           "key_correctness_proof" : <key_correctness_proof>
       }
    :return: None.
    """

    logger = logging.getLogger(__name__)
    logger.debug("prover_store_credential_offer: >>> wallet_handle: %r, credential_offer_json: %r",
                 wallet_handle,
                 credential_offer_json)

    if not hasattr(prover_store_credential_offer, "cb"):
        logger.debug("prover_store_credential_offer: Creating callback")
        prover_store_credential_offer.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_wallet_handle = c_int32(wallet_handle)
    c_credential_offer_json = c_char_p(credential_offer_json.encode('utf-8'))

    res = await do_call('indy_prover_store_credential_offer',
                        c_wallet_handle,
                        c_credential_offer_json,
                        prover_store_credential_offer.cb)

    logger.debug("prover_store_credential_offer: <<< res: %r", res)
    return res


async def prover_get_credential_offers(wallet_handle: int,
                                       filter_json: str) -> str:
    """
    Gets all stored credential offers (see prover_store_credential_offer).
    A filter can be specified to get credential offers for specific Issuer, credential_def or schema only.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param filter_json: optional filter to get credential offers for specific Issuer, credential_def or schema only only
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
    :return: A json with a list of credential offers for the filter.
        {
            [{
                "cred_def_id": string,
                "issuer_did": string,
                "nonce": string,
                "key_correctness_proof" : <key_correctness_proof>
            }]
        }
    """

    logger = logging.getLogger(__name__)
    logger.debug("prover_store_credential_offer: >>> wallet_handle: %r, filter_json: %r",
                 wallet_handle,
                 filter_json)

    if not hasattr(prover_get_credential_offers, "cb"):
        logger.debug("prover_get_credential_offers: Creating callback")
        prover_get_credential_offers.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_filter_json = c_char_p(filter_json.encode('utf-8'))

    credential_offers_json = await do_call('indy_prover_get_credential_offers',
                                           c_wallet_handle,
                                           c_filter_json,
                                           prover_get_credential_offers.cb)

    res = credential_offers_json.decode()
    logger.debug("prover_get_credential_offers: <<< res: %r", res)
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


async def prover_create_and_store_credential_req(wallet_handle: int,
                                                 prover_did: str,
                                                 credential_offer_json: str,
                                                 credential_def_json: str,
                                                 master_secret_name: str) -> str:
    """
    Creates a clam request json for the given credential offer and stores it in a secure wallet.
    The credential offer contains the information about Issuer DID and the schema (schema_key),
    The method gets public key and schema from the ledger, stores them in a wallet,
    and creates a blinded master secret for a master secret identified by a provided name.
    The master secret identified by the name must be already stored in the secure wallet
    (see prover_create_master_secret)
    The blinded master secret is a part of the credential request.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param prover_did: a DID of the prover
    :param credential_offer_json: credential offer as a json containing information about the issuer and a credential:
        {
            "cred_def_id": string,
            "rev_reg_id" : Optional<string>,
            "nonce": string,
            "key_correctness_proof" : <key_correctness_proof>
        }
    :param credential_def_json: credential definition json associated with issuer_did and schema_seq_no in
           the credential_offer
    :param master_secret_name: the name of the master secret stored in the wallet
    :return: credential request json.
        {
           "blinded_ms" : <blinded_master_secret>,
            "cred_def_id" : string,
            "issuer_did" : string,
            "prover_did" : string,
            "blinded_ms_correctness_proof" : <blinded_ms_correctness_proof>,
            "nonce": string
        }
    """

    logger = logging.getLogger(__name__)
    logger.debug(
        "prover_create_and_store_credential_req: >>> wallet_handle: %r, prover_did: %r, credential_offer_json: %r,"
        " credential_def_json: %r, master_secret_name: %r",
        wallet_handle,
        prover_did,
        credential_offer_json,
        credential_def_json,
        master_secret_name)

    if not hasattr(prover_create_and_store_credential_req, "cb"):
        logger.debug("prover_create_and_store_credential_req: Creating callback")
        prover_create_and_store_credential_req.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_prover_did = c_char_p(prover_did.encode('utf-8'))
    c_credential_offer_json = c_char_p(credential_offer_json.encode('utf-8'))
    c_credential_def_json = c_char_p(credential_def_json.encode('utf-8'))
    c_master_secret_name = c_char_p(master_secret_name.encode('utf-8'))

    credential_req_json = await do_call('indy_prover_create_and_store_credential_req',
                                        c_wallet_handle,
                                        c_prover_did,
                                        c_credential_offer_json,
                                        c_credential_def_json,
                                        c_master_secret_name,
                                        prover_create_and_store_credential_req.cb)

    res = credential_req_json.decode()
    logger.debug("prover_create_and_store_credential_req: <<< res: %r", res)
    return res


async def prover_store_credential(wallet_handle: int,
                                  id_: str,
                                  credentials_json: str,
                                  rev_reg_def_json: Optional[str]) -> None:
    """
    Updates the credential by a master secret and stores in a secure wallet.
    The credential contains the information about
    schema_key, issuer_did, revoc_reg_seq_no (see issuer_create_credential).
    Seq_no is a sequence number of the corresponding transaction in the ledger.
    The method loads a blinded secret for this key from the wallet,
    updates the credential and stores it in a wallet.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param id_: identifier by which credential will be stored in wallet
    :param credentials_json: credential json:
     {
         "values": <see credential_values_json above>,
         "signature": <signature>,
         "cred_def_id": string,
         "rev_reg_id", Optional<string>,
         "signature_correctness_proof": <signature_correctness_proof>
     }
    :param rev_reg_def_json: revocation registry definition json
    :return: None.
    """

    logger = logging.getLogger(__name__)
    logger.debug("prover_store_credential: >>> wallet_handle: %r, id: %r, credentials_json: %r, rev_reg_def_json: %r",
                 wallet_handle,
                 id_,
                 credentials_json,
                 rev_reg_def_json)

    if not hasattr(prover_store_credential, "cb"):
        logger.debug("prover_store_credential: Creating callback")
        prover_store_credential.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_wallet_handle = c_int32(wallet_handle)
    c_id = c_char_p(id_.encode('utf-8'))
    c_credentials_json = c_char_p(credentials_json.encode('utf-8'))
    c_rev_reg_def_json = c_char_p(rev_reg_def_json.encode('utf-8')) if rev_reg_def_json else None

    res = await do_call('indy_prover_store_credential',
                        c_wallet_handle,
                        c_id,
                        c_credentials_json,
                        c_rev_reg_def_json,
                        prover_store_credential.cb)

    logger.debug("prover_store_credential: <<< res: %r", res)
    return res


async def prover_get_credentials(wallet_handle: int,
                                 filter_json: str) -> str:
    """
    Gets human readable credentials according to the filter.
    If filter is NULL, then all credentials are returned.
    credentials can be filtered by Issuer, credential_def and/or Schema.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param filter_json: filter for credentials
        {
            "schema_id": string, (Optional)
            "schema_did": string, (Optional)
            "schema_name": string, (Optional)
            "schema_version": string, (Optional)
            "issuer_did": string, (Optional)
            "issuer_did": string, (Optional)
            "cred_def_id": string, (Optional)
        }
    :return: credentials json
         [{
             "referent": string,
             "values": <see credential_values_json above>,
             "issuer_did": string,
             "cred_def_id": string,
             "rev_reg_id", Optional<string>
         }]
    """

    logger = logging.getLogger(__name__)
    logger.debug("prover_get_credentials: >>> wallet_handle: %r, filter_json: %r",
                 wallet_handle,
                 filter_json)

    if not hasattr(prover_get_credentials, "cb"):
        logger.debug("prover_get_credentials: Creating callback")
        prover_get_credentials.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_filter_json = c_char_p(filter_json.encode('utf-8'))

    credentials_json = await do_call('indy_prover_get_credentials',
                                     c_wallet_handle,
                                     c_filter_json,
                                     prover_get_credentials.cb)

    res = credentials_json.decode()
    logger.debug("prover_get_credentials: <<< res: %r", res)
    return res


async def prover_get_credentials_for_proof_req(wallet_handle: int,
                                               proof_request_json: str) -> str:
    """
    Gets human readable credentials matching the given proof request.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param proof_request_json: proof request json
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
    :return: json with credentials for the given pool request.
        credential consists of referent, human-readable attributes (key-value map), cred_def_id, issuer_did and rev_reg_id.
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
    """

    logger = logging.getLogger(__name__)
    logger.debug("prover_get_credentials_for_proof_req: >>> wallet_handle: %r, proof_request_json: %r",
                 wallet_handle,
                 proof_request_json)

    if not hasattr(prover_get_credentials_for_proof_req, "cb"):
        logger.debug("prover_get_credentials_for_proof_req: Creating callback")
        prover_get_credentials_for_proof_req.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_proof_request_json = c_char_p(proof_request_json.encode('utf-8'))

    credentials_json = await do_call('indy_prover_get_credentials_for_proof_req',
                                     c_wallet_handle,
                                     c_proof_request_json,
                                     prover_get_credentials_for_proof_req.cb)

    res = credentials_json.decode()
    logger.debug("prover_get_credentials_for_proof_req: <<< res: %r", res)
    return res


async def prover_create_proof(wallet_handle: int,
                              proof_req_json: str,
                              requested_credentials_json: str,
                              schemas_json: str,
                              master_secret_name: str,
                              credential_defs_json: str,
                              rev_infos_json: str) -> str:
    """
    Creates a proof according to the given proof request
    Either a corresponding credential with optionally revealed attributes or self-attested attribute must be provided
    for each requested attribute (see indy_prover_get_credentials_for_pool_req).
    A proof request may request multiple credentials from different schemas and different issuers.
    All required schemas, public keys and revocation registries must be provided.
    The proof request also contains nonce.
    The proof contains either proof or self-attested attribute value for each requested attribute.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param proof_req_json: proof request json as come from the verifier
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
        
    :param requested_credentials_json: either a credential or self-attested attribute for each requested attribute
            {
                "requested_attr1_referent": [{"cred_id": string, "freshness": Optional<number>}, true <reveal_attr>],
                "requested_attr2_referent": [self_attested_attribute],
                "requested_attr3_referent": [{"cred_id": string, "freshness": Optional<number>}, false]
                "requested_attr4_referent": [{"cred_id": string, "freshness": Optional<number>}, true]
                "requested_predicate_1_referent": [{"cred_id": string, "freshness": Optional<number>}],
                "requested_predicate_2_referent": [{"cred_id": string, "freshness": Optional<number>}],
            }
    :param schemas_json: all schema jsons participating in the proof request
        {
            "credential1_referent_in_wallet": <schema1>,
            "credential2_referent_in_wallet": <schema2>,
            "credential3_referent_in_wallet": <schema3>,
        }
    :param master_secret_name: the name of the master secret stored in the wallet

    :param credential_defs_json: all credential definition jsons participating in the proof request
        {
            "credential1_referent_in_wallet": <credential_def1>,
            "credential2_referent_in_wallet": <credential_def2>,
            "credential3_referent_in_wallet": <credential_def3>,
        }
    :param rev_infos_json: all revocation registry jsons participating in the proof request
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
    :return: Proof json
        For each requested attribute either a proof (with optionally revealed attribute value) or
        self-attested attribute value is provided.
        Each proof is associated with a credential and corresponding schema_key, issuer_did and revoc_reg_seq_no.
        There is also aggregated proof part common for all credential proofs.
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
        }
    """

    logger = logging.getLogger(__name__)
    logger.debug("prover_create_proof: >>> wallet_handle: %r, proof_req_json: %r, requested_credentials_json: %r, "
                 "schemas_json: %r, master_secret_name: %r, credential_defs_json: %r, rev_infos_json: %r",
                 wallet_handle,
                 proof_req_json,
                 requested_credentials_json,
                 schemas_json,
                 master_secret_name,
                 credential_defs_json,
                 rev_infos_json)

    if not hasattr(prover_create_proof, "cb"):
        logger.debug("prover_create_proof: Creating callback")
        prover_create_proof.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_proof_req_json = c_char_p(proof_req_json.encode('utf-8'))
    c_requested_credentials_json = c_char_p(requested_credentials_json.encode('utf-8'))
    c_schemas_json = c_char_p(schemas_json.encode('utf-8'))
    c_master_secret_name = c_char_p(master_secret_name.encode('utf-8'))
    c_credential_defs_json = c_char_p(credential_defs_json.encode('utf-8'))
    c_rev_infos_json = c_char_p(rev_infos_json.encode('utf-8'))

    proof_json = await do_call('indy_prover_create_proof',
                               c_wallet_handle,
                               c_proof_req_json,
                               c_requested_credentials_json,
                               c_schemas_json,
                               c_master_secret_name,
                               c_credential_defs_json,
                               c_rev_infos_json,
                               prover_create_proof.cb)

    res = proof_json.decode()
    logger.debug("prover_create_proof: <<< res: %r", res)
    return res


async def verifier_verify_proof(proof_request_json: str,
                                proof_json: str,
                                schemas_json: str,
                                credential_defs_jsons: str,
                                rev_reg_defs_json: str,
                                rev_regs_json: str) -> bool:
    """
    Verifies a proof (of multiple credential).
    All required schemas, public keys and revocation registries must be provided.

    :param proof_request_json: initial proof request as sent by the verifier
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
    :param proof_json: proof json
        For each requested attribute either a proof (with optionally revealed attribute value) or
        self-attested attribute value is provided.
        Each proof is associated with a credential and corresponding schema_key, issuer_did and revoc_reg_seq_no.
        There is also aggregated proof part common for all credential proofs.
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
        }
    :param schemas_json: all schema jsons participating in the proof
        {
            "credential_proof1_referent": <schema>,
            "credential_proof2_referent": <schema>,
            "credential_proof3_referent": <schema>
        }
    :param credential_defs_jsons: all credential definition jsons participating in the proof
        {
            "credential_proof1_referent": <credential_def>,
            "credential_proof2_referent": <credential_def>,
            "credential_proof3_referent": <credential_def>
        }
    :param rev_reg_defs_json: all revocation registry jsons participating in the proof
        {
            "credential_proof1_referent": <revoc_reg>,
            "credential_proof2_referent": <revoc_reg>,
            "credential_proof3_referent": <revoc_reg>
        }
    :param rev_regs_json: all revocation registry jsons participating in the proof
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
    :return: valid: true - if signature is valid, false - otherwise
    """

    logger = logging.getLogger(__name__)
    logger.debug("verifier_verify_proof: >>> proof_request_json: %r, proof_json: %r, schemas_json: %r, "
                 "credential_defs_jsons: %r, rev_reg_defs_json: %r, rev_regs_json: %r",
                 proof_request_json,
                 proof_json,
                 schemas_json,
                 credential_defs_jsons,
                 rev_reg_defs_json,
                 rev_regs_json)

    if not hasattr(verifier_verify_proof, "cb"):
        logger.debug("verifier_verify_proof: Creating callback")
        verifier_verify_proof.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_bool))

    c_proof_request_json = c_char_p(proof_request_json.encode('utf-8'))
    c_proof_json = c_char_p(proof_json.encode('utf-8'))
    c_schemas_json = c_char_p(schemas_json.encode('utf-8'))
    c_credential_defs_jsons = c_char_p(credential_defs_jsons.encode('utf-8'))
    c_rev_reg_defs_json = c_char_p(rev_reg_defs_json.encode('utf-8'))
    c_rev_regs_json = c_char_p(rev_regs_json.encode('utf-8'))

    res = await do_call('indy_verifier_verify_proof',
                        c_proof_request_json,
                        c_proof_json,
                        c_schemas_json,
                        c_credential_defs_jsons,
                        c_rev_reg_defs_json,
                        c_rev_regs_json,
                        verifier_verify_proof.cb)

    logger.debug("verifier_verify_proof: <<< res: %r", res)
    return res


async def create_revocation_info(tails_reader_handle: int,
                                 rev_reg_def_json: str,
                                 rev_reg_delta_json: str,
                                 timestamp: int,
                                 rev_idx: int) -> str:
    logger = logging.getLogger(__name__)
    logger.debug("create_revocation_info: >>>tails_reader_handle: %r, rev_reg_def_json: %r,"
                 " rev_reg_delta_json: %r, timestamp: %r, rev_idx: %r",
                 tails_reader_handle,
                 rev_reg_def_json,
                 rev_reg_delta_json,
                 timestamp,
                 rev_idx)

    if not hasattr(create_revocation_info, "cb"):
        logger.debug("create_revocation_info: Creating callback")
        create_revocation_info.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_tails_reader_handle = c_int32(tails_reader_handle)
    c_rev_reg_def_json = c_char_p(rev_reg_def_json.encode('utf-8'))
    c_rev_reg_delta_json = c_char_p(rev_reg_delta_json.encode('utf-8'))
    c_timestamp = c_uint64(timestamp)
    c_rev_idx = c_uint32(rev_idx)

    rev_info_json = await do_call('indy_create_revocation_info',
                                  c_tails_reader_handle,
                                  c_rev_reg_def_json,
                                  c_rev_reg_delta_json,
                                  c_timestamp,
                                  c_rev_idx,
                                  create_revocation_info.cb)

    res = rev_info_json.decode()
    logger.debug("create_revocation_info: <<< res: %r", res)
    return res


async def update_revocation_info(tails_reader_handle: int,
                                 rev_info_json: str,
                                 rev_reg_def_json: str,
                                 rev_reg_delta_json: str,
                                 timestamp: int,
                                 rev_idx: int) -> str:
    logger = logging.getLogger(__name__)
    logger.debug("update_revocation_info: >>> tails_reader_handle: %r, rev_info_json: %r, "
                 "rev_reg_def_json: %r, rev_reg_delta_json: %r, timestamp: %r, rev_idx: %r",
                 tails_reader_handle,
                 rev_info_json,
                 rev_reg_def_json,
                 rev_reg_delta_json,
                 timestamp,
                 rev_idx)

    if not hasattr(update_revocation_info, "cb"):
        logger.debug("update_revocation_info: Creating callback")
        update_revocation_info.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_tails_reader_handle = c_int32(tails_reader_handle)
    c_rev_info_json = c_char_p(rev_info_json.encode('utf-8'))
    c_rev_reg_def_json = c_char_p(rev_reg_def_json.encode('utf-8'))
    c_rev_reg_delta_json = c_char_p(rev_reg_delta_json.encode('utf-8'))
    c_timestamp = c_uint64(timestamp)
    c_rev_idx = c_uint32(rev_idx)

    updated_rev_info_json = await do_call('indy_update_revocation_info',
                                          c_tails_reader_handle,
                                          c_rev_info_json,
                                          c_rev_reg_def_json,
                                          c_rev_reg_delta_json,
                                          c_timestamp,
                                          c_rev_idx,
                                          update_revocation_info.cb)

    res = updated_rev_info_json.decode()
    logger.debug("update_revocation_info: <<< res: %r", res)
    return res


async def store_revocation_info(wallet_handle: int,
                                id_: str,
                                rev_info_json: str) -> str:
    logger = logging.getLogger(__name__)
    logger.debug("store_revocation_info: >>> wallet_handle: %r, id: %r, rev_info_json: %r",
                 wallet_handle,
                 id_,
                 rev_info_json)

    if not hasattr(store_revocation_info, "cb"):
        logger.debug("store_revocation_info: Creating callback")
        store_revocation_info.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_wallet_handle = c_int32(wallet_handle)
    c_id = c_char_p(id_.encode('utf-8'))
    c_rev_info_json = c_char_p(rev_info_json.encode('utf-8'))

    res = await do_call('indy_store_revocation_info',
                        c_wallet_handle,
                        c_id,
                        c_rev_info_json,
                        store_revocation_info.cb)

    logger.debug("store_revocation_info: <<< res: %r", res)
    return res


async def get_revocation_info(wallet_handle: int,
                              id_: str,
                              timestamp: Optional[int]) -> str:
    logger = logging.getLogger(__name__)
    logger.debug("get_revocation_info: >>> wallet_handle: %r, id: %r, timestamp: %r ",
                 wallet_handle,
                 id_,
                 timestamp)

    if not hasattr(get_revocation_info, "cb"):
        logger.debug("get_revocation_info: Creating callback")
        get_revocation_info.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_id = c_char_p(id_.encode('utf-8'))
    c_timestamp = c_int64(timestamp)

    rev_info_json = await do_call('indy_get_revocation_info',
                                  c_wallet_handle,
                                  c_id,
                                  c_timestamp,
                                  get_revocation_info.cb)

    res = rev_info_json.decode()
    logger.debug("get_revocation_info: <<< res: %r", res)
    return res
