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

    :param submitter_did: DID of the submitter stored in secured Wallet.
    :param target_did: Target DID as base58-encoded string for 16 or 32 bit DID value.
    :param ver_key: Target identity verification key as base58-encoded string.
    :param alias: NYM's alias.
    :param role: Role of a user NYM record:
                             null (common USER)
                             TRUSTEE
                             STEWARD
                             TRUST_ANCHOR
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

    :param submitter_did: DID of the submitter stored in secured Wallet.
    :param target_did: Target DID as base58-encoded string for 16 or 32 bit DID value.
    :param xhash: (Optional) Hash of attribute data.
    :param raw: (Optional) Json, where key is attribute name and value is attribute value.
    :param enc: (Optional) Encrypted value attribute data.
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

    :param submitter_did: DID of the read request sender.
    :param target_did: Target DID as base58-encoded string for 16 or 32 bit DID value.
    :param xhash: (Optional) Requested attribute name.
    :param raw: (Optional) Requested attribute hash.
    :param enc: (Optional) Requested attribute encrypted value.
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
    Builds a GET_NYM request. Request to get information about a DID (NYM).

    :param submitter_did: DID of the read request sender.
    :param target_did: Target DID as base58-encoded string for 16 or 32 bit DID value.
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
    Builds a SCHEMA request. Request to add Claim's schema.

    :param submitter_did: DID of the submitter stored in secured Wallet.
    :param data: {
        attr_names: array of attribute name strings
        name: Schema's name string
        version: Schema's version string
    }
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
    Builds a GET_SCHEMA request. Request to get Claim's Schema.

    :param submitter_did: DID of the read request sender.
    :param dest: Schema Issuer's DID as base58-encoded string for 16 or 32 bit DID value.
                 It differs from submitter_did field.
    :param data: {
        name (string): Schema's name string
        version (string): Schema's version string
    }
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
    Builds an CLAIM_DEF request. Request to add a claim definition (in particular, public key),
    that Issuer creates for a particular Claim Schema.

    :param submitter_did: DID of the submitter stored in secured Wallet.
    :param xref: Sequence number of a Schema transaction the claim definition is created for.
    :param signature_type: Type of the claim definition. CL is the only supported type now.
    :param data: Dictionary with Claim Definition's data: {
        primary: primary claim public key
        revocation: revocation claim public key
    }
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
   Builds a GET_CLAIM_DEF request. Request to get a claim definition (in particular, public key),
   that Issuer creates for a particular Claim Schema.

    :param submitter_did: DID of read request sender.
    :param xref: Sequence number of a Schema transaction the claim definition is created for.
    :param signature_type: Type of the claim definition. CL is the only supported type now.
    :param origin: Claim Definition Issuer's DID as base58-encoded string for 16 or 32 bit DID value.
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
    Builds a NODE request. Request to add a new node to the pool, or updates existing in the pool.

    :param submitter_did: DID of the submitter stored in secured Wallet.
    :param target_did: Target Node's DID.  It differs from submitter_did field.
    :param data: Data associated with the Node: {
        alias: string - Node's alias
        blskey: string - (Optional) BLS multi-signature key as base58-encoded string.
        client_ip: string - (Optional) Node's client listener IP address.
        client_port: string - (Optional) Node's client listener port.
        node_ip: string - (Optional) The IP address other Nodes use to communicate with this Node.
        node_port: string - (Optional) The port other Nodes use to communicate with this Node.
        services: array<string> - (Optional) The service of the Node. VALIDATOR is the only supported one now.
    }
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
    Builds a GET_TXN request. Request to get any transaction by its seq_no.

    :param submitter_did: DID of the submitter stored in secured Wallet.
    :param data: seq_no of transaction in ledger.
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
    Builds a POOL_CONFIG request. Request to change Pool's configuration.

    :param submitter_did: DID of the submitter stored in secured Wallet.
    :param writes: Whether any write requests can be processed by the pool
                   (if false, then pool goes to read-only state). True by default.
    :param force: Whether we should apply transaction (for example, move pool to read-only state)
                  without waiting for consensus of this transaction
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
    Builds a POOL_UPGRADE request. Request to upgrade the Pool (sent by Trustee).
    It upgrades the specified Nodes (either all nodes in the Pool, or some specific ones).

    :param submitter_did: DID of the submitter stored in secured Wallet.
    :param name: Human-readable name for the upgrade.
    :param version: The version of indy-node package we perform upgrade to.
                    Must be greater than existing one (or equal if reinstall flag is True).
    :param action: Either start or cancel.
    :param _sha256: sha256 hash of the package.
    :param _timeout: (Optional) Limits upgrade time on each Node.
    :param schedule: (Optional) Schedule of when to perform upgrade on each node. Map Node DIDs to upgrade time.
    :param justification: (Optional) justification string for this particular Upgrade.
    :param reinstall: Whether it's allowed to re-install the same version. False by default.
    :param force: Whether we should apply transaction (schedule Upgrade) without waiting
                  for consensus of this transaction.
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


async def build_revoc_reg_def_request(submitter_did: str,
                                      data: str) -> str:
    """
    Builds a REVOC_REG_DEF request. Request to add the definition of revocation registry
    to an exists claim definition.

    :param submitter_did:DID of the submitter stored in secured Wallet.
    :param data: Revocation Registry specific data:
        {
             "id": string - ID of the Revocation Registry,
             "revocDefType": string - Revocation Registry type (only CL_ACCUM is supported for now),
             "tag": string - Unique descriptive ID of the Registry,
             "credDefId": string - ID of the corresponding ClaimDef,
             "value": Registry-specific data {
                 "issuanceType": string - Type of Issuance(ISSUANCE_BY_DEFAULT or ISSUANCE_ON_DEMAND),
                 "maxCredNum": number - Maximum number of credentials the Registry can serve.
                 "tailsHash": string - Hash of tails.
                 "tailsLocation": string - Location of tails file.
                 "publicKeys": <public_keys> - Registry's public key.
             }
         }
     
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_revoc_reg_def_request: >>> submitter_did: %r, data: %r", submitter_did, data)

    if not hasattr(build_revoc_reg_def_request, "cb"):
        logger.debug("build_revoc_reg_def_request: Creating callback")
        build_revoc_reg_def_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_data = c_char_p(data.encode('utf-8'))

    request_json = await do_call('indy_build_revoc_reg_def_request',
                                 c_submitter_did,
                                 c_data,
                                 build_revoc_reg_def_request.cb)

    res = request_json.decode()
    logger.debug("build_revoc_reg_def_request: <<< res: %r", res)
    return res


async def build_get_revoc_reg_def_request(submitter_did: str,
                                          rev_reg_def_id: str) -> str:
    """
    Builds a GET_REVOC_REG_DEF request. Request to get a revocation registry definition,
    that Issuer creates for a particular Credential Definition.

    :param submitter_did:DID of the submitter stored in secured Wallet.
    :param rev_reg_def_id: ID of the corresponding revocation registry definition.

    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_get_revoc_reg_def_request: >>> submitter_did: %r, rev_reg_def_id: %r", submitter_did,
                 rev_reg_def_id)

    if not hasattr(build_get_revoc_reg_def_request, "cb"):
        logger.debug("build_get_revoc_reg_def_request: Creating callback")
        build_get_revoc_reg_def_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_rev_reg_def_id = c_char_p(rev_reg_def_id.encode('utf-8'))

    request_json = await do_call('indy_build_get_revoc_reg_def_request',
                                 c_submitter_did,
                                 c_rev_reg_def_id,
                                 build_get_revoc_reg_def_request.cb)

    res = request_json.decode()
    logger.debug("build_get_revoc_reg_def_request: <<< res: %r", res)
    return res


async def build_revoc_reg_entry_request(submitter_did: str,
                                        revoc_reg_def_id: str,
                                        rev_def_type: str,
                                        value: str) -> str:
    """
    Builds a REVOC_REG_ENTRY request.  Request to add the RevocReg entry containing
    the new accumulator value and issued/revoked indices.
    This is just a delta of indices, not the whole list. So, it can be sent each time a new claim is issued/revoked.

    :param submitter_did: DID of the submitter stored in secured Wallet.
    :param revoc_reg_def_id:  ID of the corresponding RevocRegDef.
    :param rev_def_type:  Revocation Registry type (only CL_ACCUM is supported for now).
    :param value: Registry-specific data: {
           issued: array<number> - an array of issued indices.
           revoked: array<number> an array of revoked indices
           prev_accum: previous accumulator value.
           accum: current accumulator value.
        }
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_revoc_reg_entry_request: >>> submitter_did: %r, rev_def_type: %r, revoc_reg_def_id: %r, "
                 "value: %r", submitter_did, rev_def_type, revoc_reg_def_id, value)

    if not hasattr(build_revoc_reg_entry_request, "cb"):
        logger.debug("build_revoc_reg_entry_request: Creating callback")
        build_revoc_reg_entry_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_rev_def_type = c_char_p(rev_def_type.encode('utf-8'))
    c_revoc_reg_def_id = c_char_p(revoc_reg_def_id.encode('utf-8'))
    c_value = c_char_p(value.encode('utf-8'))

    request_json = await do_call('indy_build_revoc_reg_entry_request',
                                 c_submitter_did,
                                 c_revoc_reg_def_id,
                                 c_rev_def_type,
                                 c_value,
                                 build_revoc_reg_entry_request.cb)

    res = request_json.decode()
    logger.debug("build_revoc_reg_entry_request: <<< res: %r", res)
    return res


async def build_get_revoc_reg_request(submitter_did: str,
                                      revoc_reg_def_id: str,
                                      timestamp: int) -> str:
    """
    Builds a GET_REVOC_REG request. Request to get the accumulated state of the Revocation Registry
    by ID. The state is defined by the given timestamp.

    :param submitter_did: DID of the submitter stored in secured Wallet.
    :param revoc_reg_def_id:  ID of the corresponding RevocRegDef.
    :param timestamp: Requested time represented as a total number of seconds from Unix Epoch
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_get_revoc_reg_request: >>> submitter_did: %r, revoc_reg_def_id: %r, timestamp: %r",
                 submitter_did, revoc_reg_def_id, timestamp)

    if not hasattr(build_get_revoc_reg_request, "cb"):
        logger.debug("build_get_revoc_reg_request: Creating callback")
        build_get_revoc_reg_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_revoc_reg_def_id = c_char_p(revoc_reg_def_id.encode('utf-8'))
    c_timestamp = c_int64(timestamp)

    request_json = await do_call('indy_build_get_revoc_reg_request',
                                 c_submitter_did,
                                 c_revoc_reg_def_id,
                                 c_timestamp,
                                 build_get_revoc_reg_request.cb)

    res = request_json.decode()
    logger.debug("build_get_revoc_reg_request: <<< res: %r", res)
    return res


async def build_get_revoc_reg_delta_request(submitter_did: str,
                                            revoc_reg_def_id: str,
                                            from_: Optional[int],
                                            to: int) -> str:
    """
    Builds a GET_REVOC_REG_DELTA request. Request to get the delta of the accumulated state of the Revocation Registry.
    The Delta is defined by from and to timestamp fields.
    If from is not specified, then the whole state till to will be returned.

    :param submitter_did: DID of the submitter stored in secured Wallet.
    :param revoc_reg_def_id:  ID of the corresponding RevocRegDef.
    :param from_: Requested time represented as a total number of seconds from Unix Epoch
    :param to: Requested time represented as a total number of seconds from Unix Epoch
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_get_revoc_reg_delta_request: >>> submitter_did: %r, revoc_reg_def_id: %r, from: %r, to: %r",
                 submitter_did, revoc_reg_def_id, from_, to)

    if not hasattr(build_get_revoc_reg_delta_request, "cb"):
        logger.debug("build_get_revoc_reg_delta_request: Creating callback")
        build_get_revoc_reg_delta_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_revoc_reg_def_id = c_char_p(revoc_reg_def_id.encode('utf-8'))
    c_from = c_int64(from_) if from_  else -1
    c_to = c_int64(to)

    request_json = await do_call('indy_build_get_revoc_reg_delta_request',
                                 c_submitter_did,
                                 c_revoc_reg_def_id,
                                 c_from,
                                 c_to,
                                 build_get_revoc_reg_delta_request.cb)

    res = request_json.decode()
    logger.debug("build_get_revoc_reg_delta_request: <<< res: %r", res)
    return res
