from .libindy import do_call, create_cb

from typing import Optional
from ctypes import *

import logging


async def sign_and_submit_request(pool_handle: int,
                                  wallet_handle: int,
                                  submitter_did: str,
                                  request_json: str) -> str:
    logger = logging.getLogger(__name__)
    logger.debug("sign_and_submit_request: >>> pool_handle: %s, wallet_handle: %s, submitter_did: %s, request_json: %s",
                 pool_handle,
                 wallet_handle,
                 submitter_did,
                 request_json)

    if not hasattr(sign_and_submit_request, "cb"):
        logger.debug("sign_and_submit_request: Creating callback")
        sign_and_submit_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_pool_handle = c_int32(pool_handle)
    c_wallet_handle = c_int32(wallet_handle)
    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_request_json = c_char_p(request_json.encode('utf-8'))

    res = await do_call('indy_sign_and_submit_request',
                        c_pool_handle,
                        c_wallet_handle,
                        c_submitter_did,
                        c_request_json,
                        sign_and_submit_request.cb)

    logger.debug("sign_and_submit_request: <<< res: %s", res)
    return res


async def submit_request(pool_handle: int,
                         request_json: str) -> str:
    logger = logging.getLogger(__name__)
    logger.debug("submit_request: >>> pool_handle: %s, request_json: %s",
                 pool_handle,
                 request_json)

    if not hasattr(submit_request, "cb"):
        logger.debug("submit_request: Creating callback")
        submit_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_pool_handle = c_int32(pool_handle)
    c_request_json = c_char_p(request_json.encode('utf-8'))

    res = await do_call('indy_submit_request',
                        c_pool_handle,
                        c_request_json,
                        submit_request.cb)

    logger.debug("submit_request: <<< res: %s", res)
    return res


async def build_get_ddo_request(submitter_did: str,
                                target_did: str) -> str:
    logger = logging.getLogger(__name__)
    logger.debug("build_get_ddo_request: >>> submitter_did: %s, target_did: %s",
                 submitter_did,
                 target_did)

    if not hasattr(build_get_ddo_request, "cb"):
        logger.debug("build_get_ddo_request: Creating callback")
        build_get_ddo_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_target_did = c_char_p(target_did.encode('utf-8'))

    res = await do_call('indy_build_get_ddo_request',
                        c_submitter_did,
                        c_target_did,
                        build_get_ddo_request.cb)

    logger.debug("build_get_ddo_request: <<< res: %s", res)
    return res


async def build_nym_request(submitter_did: str,
                            target_did: str,
                            ver_key: Optional[str],
                            alias: Optional[str],
                            role: Optional[str]) -> str:
    logger = logging.getLogger(__name__)
    logger.debug("build_nym_request: >>> submitter_did: %s, target_did: %s, ver_key: %s, alias: %s, role: %s",
                 submitter_did,
                 target_did,
                 ver_key,
                 alias,
                 role)

    if not hasattr(build_nym_request, "cb"):
        logger.debug("build_nym_request: Creating callback")
        build_nym_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_target_did = c_char_p(target_did.encode('utf-8'))
    c_ver_key = c_char_p(ver_key.encode('utf-8')) if ver_key is not None else None
    c_alias = c_char_p(alias.encode('utf-8')) if alias is not None else None
    c_role = c_char_p(role.encode('utf-8')) if role is not None else None

    res = await do_call('indy_build_nym_request',
                        c_submitter_did,
                        c_target_did,
                        c_ver_key,
                        c_alias,
                        c_role,
                        build_nym_request.cb)

    logger.debug("build_nym_request: <<< res: %s", res)
    return res


async def build_attrib_request(submitter_did: str,
                               target_did: str,
                               xhash: Optional[str],
                               raw: Optional[str],
                               enc: Optional[str]) -> str:
    logger = logging.getLogger(__name__)
    logger.debug("build_attrib_request: >>> submitter_did: %s, target_did: %s, hash: %s, raw: %s, enc: %s",
                 submitter_did,
                 target_did,
                 xhash,
                 raw,
                 enc)

    if not hasattr(build_attrib_request, "cb"):
        logger.debug("build_attrib_request: Creating callback")
        build_attrib_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_target_did = c_char_p(target_did.encode('utf-8'))
    c_hash = c_char_p(xhash.encode('utf-8')) if xhash is not None else None
    c_raw = c_char_p(raw.encode('utf-8')) if raw is not None else None
    c_enc = c_char_p(enc.encode('utf-8')) if enc is not None else None

    res = await do_call('indy_build_attrib_request',
                        c_submitter_did,
                        c_target_did,
                        c_hash,
                        c_raw,
                        c_enc,
                        build_attrib_request.cb)

    logger.debug("build_attrib_request: <<< res: %s", res)
    return res


async def build_get_attrib_request(submitter_did: str,
                                   target_did: str,
                                   data: str) -> str:
    logger = logging.getLogger(__name__)
    logger.debug("build_get_attrib_request: >>> submitter_did: %s, target_did: %s, data: %s",
                 submitter_did,
                 target_did,
                 data)

    if not hasattr(build_get_attrib_request, "cb"):
        logger.debug("build_get_attrib_request: Creating callback")
        build_get_attrib_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_target_did = c_char_p(target_did.encode('utf-8'))
    c_data = c_char_p(data.encode('utf-8'))

    res = await do_call('indy_build_get_attrib_request',
                        c_submitter_did,
                        c_target_did,
                        c_data,
                        build_get_attrib_request.cb)

    logger.debug("build_get_attrib_request: <<< res: %s", res)
    return res


async def build_get_nym_request(submitter_did: str,
                                target_did: str) -> str:
    logger = logging.getLogger(__name__)
    logger.debug("build_get_nym_request: >>> submitter_did: %s, target_did: %s",
                 submitter_did,
                 target_did)

    if not hasattr(build_get_nym_request, "cb"):
        logger.debug("build_get_nym_request: Creating callback")
        build_get_nym_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_target_did = c_char_p(target_did.encode('utf-8'))

    res = await do_call('indy_build_get_nym_request',
                        c_submitter_did,
                        c_target_did,
                        build_get_nym_request.cb)

    logger.debug("build_get_nym_request: <<< res: %s", res)
    return res


async def build_schema_request(submitter_did: str,
                               data: str) -> str:
    logger = logging.getLogger(__name__)
    logger.debug("build_schema_request: >>> submitter_did: %s, data: %s",
                 submitter_did,
                 data)

    if not hasattr(build_schema_request, "cb"):
        logger.debug("build_schema_request: Creating callback")
        build_schema_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_data = c_char_p(data.encode('utf-8'))

    res = await do_call('indy_build_schema_request',
                        c_submitter_did,
                        c_data,
                        build_schema_request.cb)

    logger.debug("build_schema_request: <<< res: %s", res)
    return res


async def build_get_schema_request(submitter_did: str,
                                   dest: str,
                                   data: str) -> str:
    logger = logging.getLogger(__name__)
    logger.debug("build_get_schema_request: >>> submitter_did: %s, dest: %s, data: %s",
                 submitter_did,
                 dest,
                 data)

    if not hasattr(build_get_schema_request, "cb"):
        logger.debug("build_get_schema_request: Creating callback")
        build_get_schema_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_dest = c_char_p(dest.encode('utf-8'))
    c_data = c_char_p(data.encode('utf-8'))

    res = await do_call('indy_build_get_schema_request',
                        c_submitter_did,
                        c_dest,
                        c_data,
                        build_get_schema_request.cb)

    logger.debug("build_get_schema_request: <<< res: %s", res)
    return res


async def build_claim_def_txn(submitter_did: str,
                              xref: int,
                              signature_type: str,
                              data: str) -> str:
    logger = logging.getLogger(__name__)
    logger.debug("build_get_schema_request: >>> submitter_did: %s, xref: %s, signature_type: %s, data: %s",
                 submitter_did,
                 xref,
                 signature_type,
                 data)

    if not hasattr(build_claim_def_txn, "cb"):
        logger.debug("build_claim_def_txn: Creating callback")
        build_claim_def_txn.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_xref = c_int32(xref)
    c_signature_type = c_char_p(signature_type.encode('utf-8'))
    c_data = c_char_p(data.encode('utf-8'))

    res = await do_call('indy_build_claim_def_txn',
                        c_submitter_did,
                        c_xref,
                        c_signature_type,
                        c_data,
                        build_claim_def_txn.cb)

    logger.debug("build_claim_def_txn: <<< res: %s", res)
    return res


async def build_get_claim_def_txn(submitter_did: str,
                                  xref: int,
                                  signature_type: str,
                                  origin: str) -> str:
    logger = logging.getLogger(__name__)
    logger.debug("build_get_claim_def_txn: >>> submitter_did: %s, xref: %s, signature_type: %s, origin: %s",
                 submitter_did,
                 xref,
                 signature_type,
                 origin)

    if not hasattr(build_get_claim_def_txn, "cb"):
        logger.debug("build_get_claim_def_txn: Creating callback")
        build_get_claim_def_txn.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_xref = c_int32(xref)
    c_signature_type = c_char_p(signature_type.encode('utf-8'))
    c_origin = c_char_p(origin.encode('utf-8'))

    res = await do_call('indy_build_get_claim_def_txn',
                        c_submitter_did,
                        c_xref,
                        c_signature_type,
                        c_origin,
                        build_get_claim_def_txn.cb)

    logger.debug("build_get_claim_def_txn: <<< res: %s", res)
    return res


async def build_node_request(submitter_did: str,
                             target_did: str,
                             data: str) -> str:
    logger = logging.getLogger(__name__)
    logger.debug("build_node_request: >>> submitter_did: %s, target_did: %s, data: %s",
                 submitter_did,
                 target_did,
                 data)

    if not hasattr(build_node_request, "cb"):
        logger.debug("build_node_request: Creating callback")
        build_node_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_target_did = c_char_p(target_did.encode('utf-8'))
    c_data = c_char_p(data.encode('utf-8'))

    res = await do_call('indy_build_node_request',
                        c_submitter_did,
                        c_target_did,
                        c_data,
                        build_node_request.cb)

    logger.debug("build_node_request: <<< res: %s", res)
    return res


async def build_get_txn_request(submitter_did: str,
                                data: int) -> str:
    logger = logging.getLogger(__name__)
    logger.debug("build_get_txn_request: >>> submitter_did: %s, data: %s",
                 submitter_did,
                 data)

    if not hasattr(build_get_txn_request, "cb"):
        logger.debug("build_get_txn_request: Creating callback")
        build_get_txn_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_data = c_int32(data)

    res = await do_call('indy_build_get_txn_request',
                        c_submitter_did,
                        c_data,
                        build_get_txn_request.cb)

    logger.debug("build_get_txn_request: <<< res: %s", res)
    return res
