from .libindy import do_call, create_cb

from ctypes import *

import logging


async def get_schema(pool_handle: int, wallet_handle: int, submitter_did: str, id: str, options_json: str) -> str:
    """
    Gets schema json data for specified schema id.
    If data is present inside of cache, cached data is returned.
    Otherwise data is fetched from the ledger and stored inside of cache for future use.

    EXPERIMENTAL

    :param pool_handle: pool handle (created by open_pool_ledger).
    :param wallet_handle: wallet handle (created by open_wallet).
    :param submitter_did: DID of the submitter stored in secured Wallet.
    :param id: identifier of schema.
    :param options_json:
    {
        noCache: (bool, optional, false by default) Skip usage of cache,
        noUpdate: (bool, optional, false by default) Use only cached data, do not try to update.
        noStore: (bool, optional, false by default) Skip storing fresh data if updated,
        minFresh: (int, optional, -1 by default) Return cached data if not older than this many seconds. -1 means do not check age.
    }
    :return: Schema json.
    {
        id: identifier of schema
        attrNames: array of attribute name strings
        name: Schema's name string
        version: Schema's version string
        ver: Version of the Schema json
    }
    """
    logger = logging.getLogger(__name__)
    logger.debug("get_schema: >>> pool_handle: %r, wallet_handle: %r, submitter_did: %r, id: %r, options_json: %r",
                 pool_handle,
                 wallet_handle,
                 submitter_did,
                 id,
                 options_json)

    if not hasattr(get_schema, "cb"):
        logger.debug("get_schema: Creating callback")
        get_schema.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_pool_handle = c_int32(pool_handle)
    c_wallet_handle = c_int32(wallet_handle)
    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_id = c_char_p(id.encode('utf-8'))
    c_options_json = c_char_p(options_json.encode('utf-8'))

    schema_json = await do_call('indy_get_schema',
                                c_pool_handle,
                                c_wallet_handle,
                                c_submitter_did,
                                c_id,
                                c_options_json,
                                get_schema.cb)
    res = schema_json.decode()

    logger.debug("get_schema: <<< res: %r", res)
    return res


async def get_cred_def(pool_handle: int, wallet_handle: int, submitter_did: str, id: str, options_json: str) -> str:
    """
    Gets credential definition json data for specified credential definition id.
    If data is present inside of cache, cached data is returned.
    Otherwise data is fetched from the ledger and stored inside of cache for future use.

    EXPERIMENTAL

    :param pool_handle: pool handle (created by open_pool_ledger).
    :param wallet_handle: wallet handle (created by open_wallet).
    :param submitter_did: DID of the submitter stored in secured Wallet.
    :param id: identifier of credential definition.
    :param options_json:
    {
        noCache: (bool, optional, false by default) Skip usage of cache,
        noUpdate: (bool, optional, false by default) Use only cached data, do not try to update.
        noStore: (bool, optional, false by default) Skip storing fresh data if updated,
        minFresh: (int, optional, -1 by default) Return cached data if not older than this many seconds. -1 means do not check age.
    }
    :return: Credential Definition json.
    {
        id: string - identifier of credential definition
        schemaId: string - identifier of stored in ledger schema
        type: string - type of the credential definition. CL is the only supported type now.
        tag: string - allows to distinct between credential definitions for the same issuer and schema
        value: Dictionary with Credential Definition's data: {
            primary: primary credential public key,
            Optional<revocation>: revocation credential public key
        },
        ver: Version of the Credential Definition json
    }
    """
    logger = logging.getLogger(__name__)
    logger.debug("get_cred_def: >>> pool_handle: %r, wallet_handle: %r, submitter_did: %r, id: %r, options_json: %r",
                 pool_handle,
                 wallet_handle,
                 submitter_did,
                 id,
                 options_json)

    if not hasattr(get_cred_def, "cb"):
        logger.debug("get_cred_def: Creating callback")
        get_cred_def.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_pool_handle = c_int32(pool_handle)
    c_wallet_handle = c_int32(wallet_handle)
    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_id = c_char_p(id.encode('utf-8'))
    c_options_json = c_char_p(options_json.encode('utf-8'))

    cred_def_json = await do_call('indy_get_cred_def',
                                  c_pool_handle,
                                  c_wallet_handle,
                                  c_submitter_did,
                                  c_id,
                                  c_options_json,
                                  get_cred_def.cb)
    res = cred_def_json.decode()

    logger.debug("get_cred_def: <<< res: %r", res)
    return res


async def purge_schema_cache(wallet_handle: int, options_json: str) -> None:
    """
    Purge schema cache.

    EXPERIMENTAL

    :param wallet_handle: wallet handle (used for cache)
    :param options_json:
    {
        maxAge: (int, optional, -1 by default) Purge cached data if older than this many seconds. -1 means purge all.
    }
    :return: None
    """

    logger = logging.getLogger(__name__)
    logger.debug("purge_schema_cache: >>> wallet_handle: %r, options_json: %r",
                 wallet_handle,
                 options_json)

    if not hasattr(purge_schema_cache, "cb"):
        logger.debug("purge_schema_cache: Creating callback")
        purge_schema_cache.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_wallet_handle = c_int32(wallet_handle)
    c_options_json = c_char_p(options_json.encode('utf-8'))

    res = await do_call('indy_purge_schema_cache',
                        c_wallet_handle,
                        c_options_json,
                        purge_schema_cache.cb)

    logger.debug("purge_schema_cache: <<< res: %r", res)
    return res


async def purge_cred_def_cache(wallet_handle: int, options_json: str) -> None:
    """
    Purge credential definition cache.

    EXPERIMENTAL

    :param wallet_handle: wallet handle (used for cache)
    :param options_json:
    {
        maxAge: (int, optional, -1 by default) Purge cached data if older than this many seconds. -1 means purge all.
    }
    :return: None
    """

    logger = logging.getLogger(__name__)
    logger.debug("purge_cred_def_cache: >>> wallet_handle: %r, options_json: %r",
                 wallet_handle,
                 options_json)

    if not hasattr(purge_cred_def_cache, "cb"):
        logger.debug("purge_cred_def_cache: Creating callback")
        purge_cred_def_cache.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_wallet_handle = c_int32(wallet_handle)
    c_options_json = c_char_p(options_json.encode('utf-8'))

    res = await do_call('indy_purge_cred_def_cache',
                        c_wallet_handle,
                        c_options_json,
                        purge_cred_def_cache.cb)

    logger.debug("purge_cred_def_cache: <<< res: %r", res)
    return res
