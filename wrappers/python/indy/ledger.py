from .libindy import do_call, create_cb

from typing import Optional
from ctypes import *

import logging


async def sign_and_submit_request(pool_handle: int,
                                  wallet_handle: int,
                                  submitter_did: str,
                                  request_json: str) -> str:
    """
    Signs and submits request message to validator pool.

    Adds submitter information to passed request json, signs it with submitter
    sign key (see wallet_sign), and sends signed request message
    to validator pool (see write_request).

    :param pool_handle: pool handle (created by open_pool_ledger).
    :param wallet_handle: wallet handle (created by open_wallet).
    :param submitter_did: Id of Identity stored in secured Wallet.
    :param request_json: Request data json.
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("sign_and_submit_request: >>> pool_handle: %r, wallet_handle: %r, submitter_did: %r, request_json: %r",
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

    request_result = await do_call('indy_sign_and_submit_request',
                                   c_pool_handle,
                                   c_wallet_handle,
                                   c_submitter_did,
                                   c_request_json,
                                   sign_and_submit_request.cb)

    res = request_result.decode()
    logger.debug("sign_and_submit_request: <<< res: %r", res)
    return res


async def submit_request(pool_handle: int,
                         request_json: str) -> str:
    """
    Publishes request message to validator pool (no signing, unlike sign_and_submit_request).
    The request is sent to the validator pool as is. It's assumed that it's already prepared.

    :param pool_handle: pool handle (created by open_pool_ledger).
    :param request_json: Request data json.
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("submit_request: >>> pool_handle: %r, request_json: %r",
                 pool_handle,
                 request_json)

    if not hasattr(submit_request, "cb"):
        logger.debug("submit_request: Creating callback")
        submit_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_pool_handle = c_int32(pool_handle)
    c_request_json = c_char_p(request_json.encode('utf-8'))

    request_result = await do_call('indy_submit_request',
                                   c_pool_handle,
                                   c_request_json,
                                   submit_request.cb)

    res = request_result.decode()
    logger.debug("submit_request: <<< res: %r", res)
    return res


async def sign_request(wallet_handle: int,
                       submitter_did: str,
                       request_json: str) -> str:
    """
    Signs request message.

    Adds submitter information to passed request json, signs it with submitter
    sign key (see wallet_sign).

    :param wallet_handle: wallet handle (created by open_wallet).
    :param submitter_did: Id of Identity stored in secured Wallet.
    :param request_json: Request data json.
    :return: Signed request json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("sign_request: >>> wallet_handle: %r, submitter_did: %r, request_json: %r",
                 wallet_handle,
                 submitter_did,
                 request_json)

    if not hasattr(sign_request, "cb"):
        logger.debug("sign_and_submit_request: Creating callback")
        sign_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_request_json = c_char_p(request_json.encode('utf-8'))

    request_result = await do_call('indy_sign_request',
                                   c_wallet_handle,
                                   c_submitter_did,
                                   c_request_json,
                                   sign_request.cb)

    res = request_result.decode()
    logger.debug("sign_request: <<< res: %r", res)
    return res


async def build_get_ddo_request(submitter_did: str,
                                target_did: str) -> str:
    """
    Builds a request to get a DDO.

    :param submitter_did: Id of Identity stored in secured Wallet.
    :param target_did: Id of Identity stored in secured Wallet.
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_get_ddo_request: >>> submitter_did: %r, target_did: %r",
                 submitter_did,
                 target_did)

    if not hasattr(build_get_ddo_request, "cb"):
        logger.debug("build_get_ddo_request: Creating callback")
        build_get_ddo_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_target_did = c_char_p(target_did.encode('utf-8'))

    request_json = await do_call('indy_build_get_ddo_request',
                                 c_submitter_did,
                                 c_target_did,
                                 build_get_ddo_request.cb)

    res = request_json.decode()
    logger.debug("build_get_ddo_request: <<< res: %r", res)
    return res


async def build_nym_request(submitter_did: str,
                            target_did: str,
                            ver_key: Optional[str],
                            alias: Optional[str],
                            role: Optional[str]) -> str:
    """
    Builds a NYM request.

    :param submitter_did: Id of Identity stored in secured Wallet.
    :param target_did: Id of Identity stored in secured Wallet.
    :param ver_key: verification key
    :param alias: alias
    :param role: Role of a user NYM record
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_nym_request: >>> submitter_did: %r, target_did: %r, ver_key: %r, alias: %r, role: %r",
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
    c_ver_key = c_char_p(ver_key.encode('utf-8')) if ver_key else None
    c_alias = c_char_p(alias.encode('utf-8')) if alias else None
    c_role = c_char_p(role.encode('utf-8')) if role else None

    request_json = await do_call('indy_build_nym_request',
                                 c_submitter_did,
                                 c_target_did,
                                 c_ver_key,
                                 c_alias,
                                 c_role,
                                 build_nym_request.cb)

    res = request_json.decode()
    logger.debug("build_nym_request: <<< res: %r", res)
    return res


async def build_attrib_request(submitter_did: str,
                               target_did: str,
                               xhash: Optional[str],
                               raw: Optional[str],
                               enc: Optional[str]) -> str:
    """
    Builds an ATTRIB request.

    :param submitter_did: Id of Identity stored in secured Wallet.
    :param target_did: Id of Identity stored in secured Wallet.
    :param xhash: Hash of attribute data
    :param raw: represented as json, where key is attribute name and value is it's value
    :param enc: Encrypted attribute data
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_attrib_request: >>> submitter_did: %r, target_did: %r, hash: %r, raw: %r, enc: %r",
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
    c_hash = c_char_p(xhash.encode('utf-8')) if xhash else None
    c_raw = c_char_p(raw.encode('utf-8')) if raw else None
    c_enc = c_char_p(enc.encode('utf-8')) if enc else None

    request_json = await do_call('indy_build_attrib_request',
                                 c_submitter_did,
                                 c_target_did,
                                 c_hash,
                                 c_raw,
                                 c_enc,
                                 build_attrib_request.cb)

    res = request_json.decode()
    logger.debug("build_attrib_request: <<< res: %r", res)
    return res


async def build_get_attrib_request(submitter_did: str,
                                   target_did: str,
                                   raw: Optional[str],
                                   xhash: Optional[str],
                                   enc: Optional[str]) -> str:
    """
    Builds a GET_ATTRIB request.

    :param submitter_did: Id of Identity stored in secured Wallet.
    :param target_did: Id of Identity stored in secured Wallet.
    :param xhash: Hash of attribute data
    :param raw: represented as json, where key is attribute name and value is it's value
    :param enc: Encrypted attribute data
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_get_attrib_request: >>> submitter_did: %r, target_did: %r, raw: %r, xhash: %r, enc: %r",
                 submitter_did,
                 target_did,
                 raw,
                 xhash,
                 enc)

    if not hasattr(build_get_attrib_request, "cb"):
        logger.debug("build_get_attrib_request: Creating callback")
        build_get_attrib_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_target_did = c_char_p(target_did.encode('utf-8'))
    c_raw = c_char_p(raw.encode('utf-8')) if raw else None
    c_xhash = c_char_p(xhash.encode('utf-8')) if xhash else None
    c_enc = c_char_p(enc.encode('utf-8')) if enc else None

    request_json = await do_call('indy_build_get_attrib_request',
                                 c_submitter_did,
                                 c_target_did,
                                 c_raw,
                                 c_xhash,
                                 c_enc,
                                 build_get_attrib_request.cb)

    res = request_json.decode()
    logger.debug("build_get_attrib_request: <<< res: %r", res)
    return res


async def build_get_nym_request(submitter_did: str,
                                target_did: str) -> str:
    """
    Builds a GET_NYM request.

    :param submitter_did: Id of Identity stored in secured Wallet.
    :param target_did: Id of Identity stored in secured Wallet.
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_get_nym_request: >>> submitter_did: %r, target_did: %r",
                 submitter_did,
                 target_did)

    if not hasattr(build_get_nym_request, "cb"):
        logger.debug("build_get_nym_request: Creating callback")
        build_get_nym_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_target_did = c_char_p(target_did.encode('utf-8'))

    request_json = await do_call('indy_build_get_nym_request',
                                 c_submitter_did,
                                 c_target_did,
                                 build_get_nym_request.cb)

    res = request_json.decode()
    logger.debug("build_get_nym_request: <<< res: %r", res)
    return res


async def build_schema_request(submitter_did: str,
                               data: str) -> str:
    """
    Builds a SCHEMA request.

    :param submitter_did: Id of Identity stored in secured Wallet.
    :param data: name, version, type, attr_names (ip, port, keys)
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_schema_request: >>> submitter_did: %r, data: %r",
                 submitter_did,
                 data)

    if not hasattr(build_schema_request, "cb"):
        logger.debug("build_schema_request: Creating callback")
        build_schema_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_data = c_char_p(data.encode('utf-8'))

    request_json = await do_call('indy_build_schema_request',
                                 c_submitter_did,
                                 c_data,
                                 build_schema_request.cb)

    res = request_json.decode()
    logger.debug("build_schema_request: <<< res: %r", res)
    return res


async def build_get_schema_request(submitter_did: str,
                                   dest: str,
                                   data: str) -> str:
    """
    Builds a GET_SCHEMA request.

    :param submitter_did: Id of Identity stored in secured Wallet.
    :param dest: Id of Identity stored in secured Wallet.
    :param data: name, version
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_get_schema_request: >>> submitter_did: %r, dest: %r, data: %r",
                 submitter_did,
                 dest,
                 data)

    if not hasattr(build_get_schema_request, "cb"):
        logger.debug("build_get_schema_request: Creating callback")
        build_get_schema_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_dest = c_char_p(dest.encode('utf-8'))
    c_data = c_char_p(data.encode('utf-8'))

    request_json = await do_call('indy_build_get_schema_request',
                                 c_submitter_did,
                                 c_dest,
                                 c_data,
                                 build_get_schema_request.cb)

    res = request_json.decode()
    logger.debug("build_get_schema_request: <<< res: %r", res)
    return res


async def build_claim_def_txn(submitter_did: str,
                              xref: int,
                              signature_type: str,
                              data: str) -> str:
    """
    Builds an CLAIM_DEF request.

    :param submitter_did: Id of Identity stored in secured Wallet.
    :param xref: Seq. number of schema
    :param signature_type: signature type (only CL supported now)
    :param data: components of a key in json: N, R, S, Z
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_get_schema_request: >>> submitter_did: %r, xref: %r, signature_type: %r, data: %r",
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

    request_result = await do_call('indy_build_claim_def_txn',
                                   c_submitter_did,
                                   c_xref,
                                   c_signature_type,
                                   c_data,
                                   build_claim_def_txn.cb)

    res = request_result.decode()
    logger.debug("build_claim_def_txn: <<< res: %r", res)
    return res


async def build_get_claim_def_txn(submitter_did: str,
                                  xref: int,
                                  signature_type: str,
                                  origin: str) -> str:
    """
    Builds a GET_CLAIM_DEF request.

    :param submitter_did: Id of Identity stored in secured Wallet.
    :param xref: Seq. number of schema
    :param signature_type: signature type (only CL supported now)
    :param origin: issuer did
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_get_claim_def_txn: >>> submitter_did: %r, xref: %r, signature_type: %r, origin: %r",
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

    request_json = await do_call('indy_build_get_claim_def_txn',
                                 c_submitter_did,
                                 c_xref,
                                 c_signature_type,
                                 c_origin,
                                 build_get_claim_def_txn.cb)

    res = request_json.decode()
    logger.debug("build_get_claim_def_txn: <<< res: %r", res)
    return res


async def build_node_request(submitter_did: str,
                             target_did: str,
                             data: str) -> str:
    """
    Builds a NODE request.

    :param submitter_did: Id of Identity stored in secured Wallet.
    :param target_did: Id of Identity stored in secured Wallet.
    :param data: id of a target NYM record
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_node_request: >>> submitter_did: %r, target_did: %r, data: %r",
                 submitter_did,
                 target_did,
                 data)

    if not hasattr(build_node_request, "cb"):
        logger.debug("build_node_request: Creating callback")
        build_node_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_target_did = c_char_p(target_did.encode('utf-8'))
    c_data = c_char_p(data.encode('utf-8'))

    request_json = await do_call('indy_build_node_request',
                                 c_submitter_did,
                                 c_target_did,
                                 c_data,
                                 build_node_request.cb)

    res = request_json.decode()
    logger.debug("build_node_request: <<< res: %r", res)
    return res


async def build_get_txn_request(submitter_did: str,
                                data: int) -> str:
    """
    Builds a GET_TXN request.

    :param submitter_did: Id of Identity stored in secured Wallet.
    :param data: seq_no of transaction in ledger
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_get_txn_request: >>> submitter_did: %r, data: %r",
                 submitter_did,
                 data)

    if not hasattr(build_get_txn_request, "cb"):
        logger.debug("build_get_txn_request: Creating callback")
        build_get_txn_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_data = c_int32(data)

    request_json = await do_call('indy_build_get_txn_request',
                                 c_submitter_did,
                                 c_data,
                                 build_get_txn_request.cb)

    res = request_json.decode()
    logger.debug("build_get_txn_request: <<< res: %r", res)
    return res


async def build_pool_config_request(submitter_did: str,
                                    writes: bool,
                                    force: bool) -> str:
    """
    Builds a POOL_CONFIG request.

    :param submitter_did: Id of Identity stored in secured Wallet.
    :param writes:
    :param force:
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_pool_config_request: >>> submitter_did: %r, writes: %r, force: %r",
                 submitter_did,
                 writes,
                 force)

    if not hasattr(build_pool_config_request, "cb"):
        logger.debug("build_pool_config_request: Creating callback")
        build_pool_config_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_writes = c_bool(writes)
    c_force = c_bool(force)

    request_json = await do_call('indy_build_pool_config_request',
                                 c_submitter_did,
                                 c_writes,
                                 c_force,
                                 build_pool_config_request.cb)

    res = request_json.decode()
    logger.debug("build_pool_config_request: <<< res: %r", res)
    return res


async def build_pool_upgrade_request(submitter_did: str,
                                     name: str,
                                     version: str,
                                     action: str,
                                     _sha256: str,
                                     _timeout: Optional[int],
                                     schedule: Optional[str],
                                     justification: Optional[str],
                                     reinstall: bool,
                                     force: bool) -> str:
    """
    Builds a POOL_UPGRADE request.

    :param submitter_did: Id of Identity stored in secured Wallet.
    :param name:
    :param version:
    :param action:
    :param _sha256:
    :param _timeout:
    :param schedule:
    :param justification:
    :param reinstall:
    :param force:
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_pool_upgrade_request: >>> submitter_did: %r, name: %r, version: %r, action: %r, _sha256: %r, "
                 "timeout: %r, schedule: %r, justification: %r, reinstall: %r, force: %r",
                 submitter_did, name, version, action, _sha256, _timeout, schedule, justification, reinstall, force)

    if not hasattr(build_pool_upgrade_request, "cb"):
        logger.debug("build_pool_upgrade_request: Creating callback")
        build_pool_upgrade_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_name = c_char_p(name.encode('utf-8'))
    c_version = c_char_p(version.encode('utf-8'))
    c_action = c_char_p(action.encode('utf-8'))
    c_sha256 = c_char_p(_sha256.encode('utf-8'))
    c_timeout = c_int32(_timeout) if _timeout else c_int32(-1)
    c_schedule = c_char_p(schedule.encode('utf-8')) if schedule else None
    c_justification = c_char_p(justification.encode('utf-8')) if justification else None
    c_reinstall = c_bool(reinstall)
    c_force = c_bool(force)

    request_json = await do_call('indy_build_pool_upgrade_request',
                                 c_submitter_did,
                                 c_name,
                                 c_version,
                                 c_action,
                                 c_sha256,
                                 c_timeout,
                                 c_schedule,
                                 c_justification,
                                 c_reinstall,
                                 c_force,
                                 build_pool_upgrade_request.cb)

    res = request_json.decode()
    logger.debug("build_pool_upgrade_request: <<< res: %r", res)
    return res
