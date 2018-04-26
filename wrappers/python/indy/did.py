from typing import Optional

from .libindy import do_call, create_cb

from ctypes import *

import logging


async def create_and_store_my_did(wallet_handle: int,
                                  did_json: str) -> (str, str):
    """
    Creates keys (signing and encryption keys) for a new
    DID (owned by the caller of the library).
    Identity's DID must be either explicitly provided, or taken as the first 16 bit of verkey.
    Saves the Identity DID with keys in a secured Wallet, so that it can be used to sign
    and encrypt transactions.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param did_json: Identity information as json. Example:
        {
            "did": string, (optional;
                    if not provided and cid param is false then the first 16 bit of the verkey will be
                    used as a new DID;
                    if not provided and cid is true then the full verkey will be used as a new DID;
                    if provided, then keys will be replaced - key rotation use case)
            "seed": string, (optional; if not provide then a random one will be created)
            "crypto_type": string, (optional; if not set then ed25519 curve is used;
                      currently only 'ed25519' value is supported for this field)
            "cid": bool, (optional; if not set then false is used;)
        }
    :return: DID and verkey (for verification of signature)
    """

    logger = logging.getLogger(__name__)
    logger.debug("create_and_store_my_did: >>> wallet_handle: %r, did_json: %r",
                 wallet_handle,
                 did_json)

    if not hasattr(create_and_store_my_did, "cb"):
        logger.debug("create_wallet: Creating callback")
        create_and_store_my_did.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_did_json = c_char_p(did_json.encode('utf-8'))

    did, verkey = await do_call('indy_create_and_store_my_did',
                                c_wallet_handle,
                                c_did_json,
                                create_and_store_my_did.cb)

    res = (did.decode(), verkey.decode())

    logger.debug("create_and_store_my_did: <<< res: %r", res)
    return res


async def replace_keys_start(wallet_handle: int,
                             did: str,
                             identity_json: str) -> str:
    """
    Generated new keys (signing and encryption keys) for an existing
    DID (owned by the caller of the library).

    :param wallet_handle: wallet handler (created by open_wallet).
    :param did: signing DID
    :param identity_json: Identity information as json. Example:
        {
            "seed": string, (optional; if not provide then a random one will be created)
            "crypto_type": string, (optional; if not set then ed25519 curve is used;
                      currently only 'ed25519' value is supported for this field)
        }
    :return: verkey
    """

    logger = logging.getLogger(__name__)
    logger.debug("replace_keys_start: >>> wallet_handle: %r, did: %r, identity_json: %r",
                 wallet_handle,
                 did,
                 identity_json)

    if not hasattr(replace_keys_start, "cb"):
        logger.debug("replace_keys_start: Creating callback")
        replace_keys_start.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_did = c_char_p(did.encode('utf-8'))
    c_identity_json = c_char_p(identity_json.encode('utf-8'))

    verkey = await do_call('indy_replace_keys_start',
                           c_wallet_handle,
                           c_did,
                           c_identity_json,
                           replace_keys_start.cb)

    res = verkey.decode()

    logger.debug("replace_keys_start: <<< res: %r", res)
    return res


async def replace_keys_apply(wallet_handle: int,
                             did: str) -> None:
    """
    Apply temporary keys as main for an existing DID (owned by the caller of the library).

    :param wallet_handle: wallet handler (created by open_wallet).
    :param did: The DID to resolve key.
    :return: Error code
    """

    logger = logging.getLogger(__name__)
    logger.debug("replace_keys_apply: >>> wallet_handle: %r, did: %r, identity_json: %r",
                 wallet_handle,
                 did)

    if not hasattr(replace_keys_apply, "cb"):
        logger.debug("replace_keys_apply: Creating callback")
        replace_keys_apply.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_wallet_handle = c_int32(wallet_handle)
    c_did = c_char_p(did.encode('utf-8'))

    await do_call('indy_replace_keys_apply',
                  c_wallet_handle,
                  c_did,
                  replace_keys_apply.cb)

    logger.debug("replace_keys_apply: <<<")


async def store_their_did(wallet_handle: int,
                          identity_json: str) -> None:
    """
    Saves their DID for a pairwise connection in a secured Wallet,
    so that it can be used to verify transaction.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param identity_json: Identity information as json. Example:
        {
           "did": string, (required)
           "verkey": string (optional, if only pk is provided),
           "crypto_type": string, (optional; if not set then ed25519 curve is used;
                  currently only 'ed25519' value is supported for this field)
        }
    :return: None
    """

    logger = logging.getLogger(__name__)
    logger.debug("store_their_did: >>> wallet_handle: %r, identity_json: %r",
                 wallet_handle,
                 identity_json)

    if not hasattr(store_their_did, "cb"):
        logger.debug("store_their_did: Creating callback")
        store_their_did.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_wallet_handle = c_int32(wallet_handle)
    c_identity_json = c_char_p(identity_json.encode('utf-8'))

    res = await do_call('indy_store_their_did',
                        c_wallet_handle,
                        c_identity_json,
                        store_their_did.cb)

    logger.debug("store_their_did: <<< res: %r", res)
    return res

async def create_key(wallet_handle: int,
                     key_json: str) -> str:
    """
    Creates keys pair and stores in the wallet.

    :param wallet_handle: Wallet handle (created by open_wallet).
    :param key_json: Key information as json. Example:
        {
            "seed": string, // Optional (if not set random one will be used);
                    Seed information that allows deterministic key creation.
            "crypto_type": string, // Optional (if not set then ed25519 curve is used);
                    Currently only 'ed25519' value is supported for this field.
        }
    :return: verkey: Ver key of generated key pair, also used as key identifier
    """

    logger = logging.getLogger(__name__)
    logger.debug("create_key: >>> wallet_handle: %r, key_json: %r",
                 wallet_handle,
                 key_json)

    if not hasattr(create_key, "cb"):
        logger.debug("create_key: Creating callback")
        create_key.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_key_json = c_char_p(key_json.encode('utf-8'))

    verkey = await do_call('indy_create_key',
                           c_wallet_handle,
                           c_key_json,
                           create_key.cb)

    res = verkey.decode()

    logger.debug("create_key: <<< res: %r", res)
    return res


async def set_key_metadata(wallet_handle: int,
                           verkey: str,
                           metadata: str) -> None:
    """
    Creates keys pair and stores in the wallet.

    :param wallet_handle: Wallet handle (created by open_wallet).
    :param verkey: the key (verkey, key id) to store metadata.
    :param metadata: the meta information that will be store with the key.
    :return: Error code
    """

    logger = logging.getLogger(__name__)
    logger.debug("set_key_metadata: >>> wallet_handle: %r, verkey: %r, metadata: %r",
                 wallet_handle,
                 verkey,
                 metadata)

    if not hasattr(set_key_metadata, "cb"):
        logger.debug("set_key_metadata: Creating callback")
        set_key_metadata.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_wallet_handle = c_int32(wallet_handle)
    c_verkey = c_char_p(verkey.encode('utf-8'))
    c_metadata = c_char_p(metadata.encode('utf-8'))

    await do_call('indy_set_key_metadata',
                  c_wallet_handle,
                  c_verkey,
                  c_metadata,
                  set_key_metadata.cb)

    logger.debug("create_key: <<<")


async def get_key_metadata(wallet_handle: int,
                           verkey: str) -> str:
    """
    Retrieves the meta information for the giving key in the wallet.

    :param wallet_handle: Wallet handle (created by open_wallet).
    :param verkey: The key (verkey, key id) to retrieve metadata.
    :return: metadata: The meta information stored with the key; Can be null if no metadata was saved for this key.
    """

    logger = logging.getLogger(__name__)
    logger.debug("get_key_metadata: >>> wallet_handle: %r, verkey: %r",
                 wallet_handle,
                 verkey)

    if not hasattr(get_key_metadata, "cb"):
        logger.debug("get_key_metadata: Creating callback")
        get_key_metadata.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_verkey = c_char_p(verkey.encode('utf-8'))

    metadata = await do_call('indy_get_key_metadata',
                             c_wallet_handle,
                             c_verkey,
                             get_key_metadata.cb)

    res = metadata.decode()

    logger.debug("get_key_metadata: <<< res: %r", res)
    return res


async def key_for_did(pool_handle: int,
                      wallet_handle: int,
                      did: str) -> str:
    """
    Returns ver key (key id) for the given DID.

    "key_for_did" call follow the idea that we resolve information about their DID from
    the ledger with cache in the local wallet. The "open_wallet" call has freshness parameter
    that is used for checking the freshness of cached pool value.

    Note if you don't want to resolve their DID info from the ledger you can use
    "key_for_local_did" call instead that will look only to local wallet and skip
    freshness checking.

    Note that "create_and_store_my_did" makes similar wallet record as "create_key".
    As result we can use returned ver key in all generic crypto and messaging functions.

    :param pool_handle: Pool handle (created by open_pool).
    :param wallet_handle: Wallet handle (created by open_wallet).
    :param did: The DID to resolve key.
    :return: key:   The DIDs ver key (key id).
    """

    logger = logging.getLogger(__name__)
    logger.debug("key_for_did: >>> pool_handle: %r, wallet_handle: %r, did: %r",
                 pool_handle,
                 wallet_handle,
                 did)

    if not hasattr(key_for_did, "cb"):
        logger.debug("key_for_did: Creating callback")
        key_for_did.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_pool_handle = c_int32(pool_handle)
    c_wallet_handle = c_int32(wallet_handle)
    c_did = c_char_p(did.encode('utf-8'))

    key = await do_call('indy_key_for_did',
                        c_pool_handle,
                        c_wallet_handle,
                        c_did,
                        key_for_did.cb)

    res = key.decode()

    logger.debug("key_for_did: <<< res: %r", res)
    return res


async def key_for_local_did(wallet_handle: int,
                            did: str) -> str:
    """
    Returns ver key (key id) for the given DID.

    "key_for_local_did" call looks data stored in the local wallet only and skips freshness checking.

    Note if you want to get fresh data from the ledger you can use "key_for_did" call
    instead.

    Note that "create_and_store_my_did" makes similar wallet record as "create_key".
    As result we can use returned ver key in all generic crypto and messaging functions.

    :param wallet_handle: Wallet handle (created by open_wallet).
    :param did: The DID to resolve key.
    :return: key: The DIDs ver key (key id).
    """

    logger = logging.getLogger(__name__)
    logger.debug("key_for_local_did: >>> wallet_handle: %r, did: %r",
                 wallet_handle,
                 did)

    if not hasattr(key_for_local_did, "cb"):
        logger.debug("key_for_local_did: Creating callback")
        key_for_local_did.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_did = c_char_p(did.encode('utf-8'))

    key = await do_call('indy_key_for_local_did',
                        c_wallet_handle,
                        c_did,
                        key_for_local_did.cb)

    res = key.decode()

    logger.debug("key_for_local_did: <<< res: %r", res)
    return res


async def set_endpoint_for_did(wallet_handle: int,
                               did: str,
                               address: str,
                               transport_key: str) -> None:
    """
    Set/replaces endpoint information for the given DID.

    :param wallet_handle: Wallet handle (created by open_wallet).
    :param did: The DID to resolve endpoint.
    :param address: The DIDs endpoint address.
    :param transport_key: The DIDs transport key (ver key, key id).
    :return: Error code
    """

    logger = logging.getLogger(__name__)
    logger.debug("set_endpoint_for_did: >>> wallet_handle: %r, did: %r, address: %r, transport_key: %r",
                 wallet_handle,
                 did,
                 address,
                 transport_key)

    if not hasattr(set_endpoint_for_did, "cb"):
        logger.debug("set_endpoint_for_did: Creating callback")
        set_endpoint_for_did.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_wallet_handle = c_int32(wallet_handle)
    c_did = c_char_p(did.encode('utf-8'))
    c_address = c_char_p(address.encode('utf-8'))
    c_transport_key = c_char_p(transport_key.encode('utf-8'))

    await do_call('indy_set_endpoint_for_did',
                  c_wallet_handle,
                  c_did,
                  c_address,
                  c_transport_key,
                  set_endpoint_for_did.cb)

    logger.debug("set_endpoint_for_did: <<<")


async def get_endpoint_for_did(wallet_handle: int,
                               pool_handle: int,
                               did: str) -> (str, Optional[str]):
    """
    Returns endpoint information for the given DID.

    :param wallet_handle: Wallet handle (created by open_wallet).
    :param pool_handle: Pool handle (created by open_pool).
    :param did: The DID to resolve endpoint.
    :return: (endpoint, transport_vk)
    """

    logger = logging.getLogger(__name__)
    logger.debug("get_endpoint_for_did: >>> wallet_handle: %r, pool_handle: %r, did: %r",
                 wallet_handle,
                 pool_handle,
                 did)

    if not hasattr(get_endpoint_for_did, "cb"):
        logger.debug("get_endpoint_for_did: Creating callback")
        get_endpoint_for_did.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_pool_handle = c_int32(pool_handle)
    c_did = c_char_p(did.encode('utf-8'))

    endpoint, transport_vk = await do_call('indy_get_endpoint_for_did',
                                           c_wallet_handle,
                                           c_pool_handle,
                                           c_did,
                                           get_endpoint_for_did.cb)

    endpoint = endpoint.decode()
    transport_vk = transport_vk.decode() if transport_vk is not None else None
    res = (endpoint, transport_vk)

    logger.debug("get_endpoint_for_did: <<< res: %r", res)
    return res


async def set_did_metadata(wallet_handle: int,
                           did: str,
                           metadata: str) -> None:
    """
    Saves/replaces the meta information for the giving DID in the wallet.

    :param wallet_handle: Wallet handle (created by open_wallet).
    :param did: the DID to store metadata.
    :param metadata: the meta information that will be store with the DID.
    :return: Error code
    """

    logger = logging.getLogger(__name__)
    logger.debug("set_did_metadata: >>> wallet_handle: %r, did: %r, metadata: %r",
                 wallet_handle,
                 did,
                 metadata)

    if not hasattr(set_did_metadata, "cb"):
        logger.debug("set_did_metadata: Creating callback")
        set_did_metadata.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_wallet_handle = c_int32(wallet_handle)
    c_did = c_char_p(did.encode('utf-8'))
    c_metadata = c_char_p(metadata.encode('utf-8'))

    await do_call('indy_set_did_metadata',
                  c_wallet_handle,
                  c_did,
                  c_metadata,
                  set_did_metadata.cb)

    logger.debug("set_did_metadata: <<<")


async def get_did_metadata(wallet_handle: int,
                           did: str) -> str:
    """
    Retrieves the meta information for the giving DID in the wallet.

    :param wallet_handle: Wallet handle (created by open_wallet).
    :param did: The DID to retrieve metadata.
    :return: metadata: The meta information stored with the DID; Can be null if no metadata was saved for this DID.
    """

    logger = logging.getLogger(__name__)
    logger.debug("get_did_metadata: >>> wallet_handle: %r, did: %r",
                 wallet_handle,
                 did)

    if not hasattr(get_did_metadata, "cb"):
        logger.debug("get_did_metadata: Creating callback")
        get_did_metadata.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_did = c_char_p(did.encode('utf-8'))

    metadata = await do_call('indy_get_did_metadata',
                             c_wallet_handle,
                             c_did,
                             get_did_metadata.cb)

    res = metadata.decode()

    logger.debug("get_did_metadata: <<< res: %r", res)
    return res


async def get_my_did_with_meta(wallet_handle: int, did: str) -> str:
    """
    Get DID metadata and verkey stored in the wallet.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param did: The DID to retrieve metadata.
    :return: DID with verkey and metadata.
    """

    logger = logging.getLogger(__name__)
    logger.debug("get_my_did_with_meta: >>> wallet_handle: %r, did: %r",
                 wallet_handle,
                 did)

    if not hasattr(get_my_did_with_meta, "cb"):
        logger.debug("get_my_did_with_meta: Creating callback")
        get_my_did_with_meta.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_did = c_char_p(did.encode('utf-8'))

    did_with_meta = await do_call('indy_get_my_did_with_meta',
                                  c_wallet_handle,
                                  c_did,
                                  get_my_did_with_meta.cb)

    res = did_with_meta.decode()

    logger.debug("get_my_did_with_meta: <<< res: %r", res)
    return res


async def list_my_dids_with_meta(wallet_handle: int) -> str:
    """
    List DIDs and metadata stored in the wallet.

    :param wallet_handle: wallet handler (created by open_wallet).
    :return: List of DIDs with verkeys and meta data.
    """

    logger = logging.getLogger(__name__)
    logger.debug("list_my_dids_with_meta: >>> wallet_handle: %r",
                 wallet_handle)

    if not hasattr(list_my_dids_with_meta, "cb"):
        logger.debug("list_my_dids_with_meta: Creating callback")
        list_my_dids_with_meta.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)

    dids_with_meta = await do_call('indy_list_my_dids_with_meta',
                                    c_wallet_handle,
                                    list_my_dids_with_meta.cb)

    res = dids_with_meta.decode()

    logger.debug("list_my_dids_with_meta: <<< res: %r", res)
    return res


async def abbreviate_verkey(did: str,
                          full_verkey: str) -> str:
    """
    Retrieves abbreviated verkey if it is possible otherwise return full verkey.

    :param did: The DID.
    :param full_verkey: The DIDs verification key,
    :return: metadata: Either abbreviated or full verkey.
    """

    logger = logging.getLogger(__name__)
    logger.debug("abbreviate_verkey: >>> did: %r, full_verkey: %r",
                 did, full_verkey)

    if not hasattr(abbreviate_verkey, "cb"):
        logger.debug("abbreviate_verkey: Creating callback")
        abbreviate_verkey.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_did = c_char_p(did.encode('utf-8'))
    c_full_verkey = c_char_p(full_verkey.encode('utf-8'))

    metadata = await do_call('indy_abbreviate_verkey',
                             c_did,
                             c_full_verkey,
                             abbreviate_verkey.cb)

    res = metadata.decode()

    logger.debug("abbreviate_verkey: <<< res: %r", res)
    return res
