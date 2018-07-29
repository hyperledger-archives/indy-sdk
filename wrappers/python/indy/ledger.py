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


async def multi_sign_request(wallet_handle: int,
                             submitter_did: str,
                             request_json: str) -> str:
    """
    Multi signs request message.

    Adds submitter information to passed request json, signs it with submitter
    sign key (see wallet_sign).

    :param wallet_handle: wallet handle (created by open_wallet).
    :param submitter_did: Id of Identity stored in secured Wallet.
    :param request_json: Request data json.
    :return: Signed request json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("multi_sign_request: >>> wallet_handle: %r, submitter_did: %r, request_json: %r",
                 wallet_handle,
                 submitter_did,
                 request_json)

    if not hasattr(multi_sign_request, "cb"):
        logger.debug("sign_and_submit_request: Creating callback")
        multi_sign_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_request_json = c_char_p(request_json.encode('utf-8'))

    request_result = await do_call('indy_multi_sign_request',
                                   c_wallet_handle,
                                   c_submitter_did,
                                   c_request_json,
                                   multi_sign_request.cb)

    res = request_result.decode()
    logger.debug("multi_sign_request: <<< res: %r", res)
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
                             empty string to reset role
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
    c_ver_key = c_char_p(ver_key.encode('utf-8')) if ver_key is not None else None
    c_alias = c_char_p(alias.encode('utf-8')) if alias is not None else None
    c_role = c_char_p(role.encode('utf-8')) if role is not None else None

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
    Builds an ATTRIB request. Request to add attribute to a NYM record.

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
    c_hash = c_char_p(xhash.encode('utf-8')) if xhash is not None else None
    c_raw = c_char_p(raw.encode('utf-8')) if raw is not None else None
    c_enc = c_char_p(enc.encode('utf-8')) if enc is not None else None

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
    Builds a GET_ATTRIB request. Request to get information about an Attribute for the specified DID.

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
    c_raw = c_char_p(raw.encode('utf-8')) if raw is not None else None
    c_xhash = c_char_p(xhash.encode('utf-8')) if xhash is not None else None
    c_enc = c_char_p(enc.encode('utf-8')) if enc is not None else None

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
    Builds a SCHEMA request. Request to add Credential's schema.

    :param submitter_did: DID of the submitter stored in secured Wallet.
    :param data: Credential schema.
                 {
                     id: identifier of schema
                     attrNames: array of attribute name strings
                     name: Schema's name string
                     version: Schema's version string,
                     ver: Version of the Schema json
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
                                   id_: str) -> str:
    """
    Builds a GET_SCHEMA request. Request to get Credential's Schema.

    :param submitter_did: DID of the read request sender.
    :param id_: Schema Id in ledger
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_get_schema_request: >>> submitter_did: %r, id: %r",
                 submitter_did,
                 id_)

    if not hasattr(build_get_schema_request, "cb"):
        logger.debug("build_get_schema_request: Creating callback")
        build_get_schema_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_id = c_char_p(id_.encode('utf-8'))

    request_json = await do_call('indy_build_get_schema_request',
                                 c_submitter_did,
                                 c_id,
                                 build_get_schema_request.cb)

    res = request_json.decode()
    logger.debug("build_get_schema_request: <<< res: %r", res)
    return res


async def parse_get_schema_response(get_schema_response: str) -> (str, str):
    """
    Parse a GET_SCHEMA response to get Schema in the format compatible with Anoncreds API

    :param get_schema_response: response of GET_SCHEMA request.
    :return: Schema Id and Schema json.
     {
         id: identifier of schema
         attrNames: array of attribute name strings
         name: Schema's name string
         version: Schema's version string
         ver: Version of the Schema json
     }
    """

    logger = logging.getLogger(__name__)
    logger.debug("parse_get_schema_response: >>> get_schema_response: %r", get_schema_response)

    if not hasattr(parse_get_schema_response, "cb"):
        logger.debug("parse_get_schema_response: Creating callback")
        parse_get_schema_response.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p, c_char_p))

    c_get_schema_response = c_char_p(get_schema_response.encode('utf-8'))

    (schema_id, schema_json) = await do_call('indy_parse_get_schema_response',
                                             c_get_schema_response,
                                             parse_get_schema_response.cb)

    res = (schema_id.decode(), schema_json.decode())
    logger.debug("parse_get_schema_response: <<< res: %r", res)
    return res


async def build_cred_def_request(submitter_did: str,
                                 data: str) -> str:
    """
    Builds an CRED_DEF request. Request to add a credential definition (in particular, public key),
    that Issuer creates for a particular Credential Schema.

    :param submitter_did: DID of the submitter stored in secured Wallet.
    :param data: credential definition json
                 {
                     id: string - identifier of credential definition
                     schemaId: string - identifier of stored in ledger schema
                     type: string - type of the credential definition. CL is the only supported type now.
                     tag: string - allows to distinct between credential definitions for the same issuer and schema
                     value: Dictionary with Credential Definition's data: {
                         primary: primary credential public key,
                         Optional<revocation>: revocation credential public key
                     },
                     ver: Version of the CredDef json
                 }
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_cred_def_request: >>> submitter_did: %r, data: %r",
                 submitter_did,
                 data)

    if not hasattr(build_cred_def_request, "cb"):
        logger.debug("build_cred_def_request: Creating callback")
        build_cred_def_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_data = c_char_p(data.encode('utf-8'))

    request_result = await do_call('indy_build_cred_def_request',
                                   c_submitter_did,
                                   c_data,
                                   build_cred_def_request.cb)

    res = request_result.decode()
    logger.debug("build_cred_def_request: <<< res: %r", res)
    return res


async def build_get_cred_def_request(submitter_did: str,
                                     id_: str) -> str:
    """
   Builds a GET_CRED_DEF request. Request to get a credential definition (in particular, public key),
   that Issuer creates for a particular Credential Schema.

    :param submitter_did: DID of read request sender.
    :param id_: Credential Definition Id in ledger.
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_get_cred_def_request: >>> submitter_did: %r, id: %r",
                 submitter_did,
                 id_)

    if not hasattr(build_get_cred_def_request, "cb"):
        logger.debug("build_get_cred_def_request: Creating callback")
        build_get_cred_def_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_id = c_char_p(id_.encode('utf-8'))

    request_json = await do_call('indy_build_get_cred_def_request',
                                 c_submitter_did,
                                 c_id,
                                 build_get_cred_def_request.cb)

    res = request_json.decode()
    logger.debug("build_get_cred_def_request: <<< res: %r", res)
    return res


async def parse_get_cred_def_response(get_cred_def_response: str) -> (str, str):
    """
    Parse a GET_CRED_DEF response to get Credential Definition in the format compatible with Anoncreds API.

    :param get_cred_def_response: response of GET_CRED_DEF request.
    :return: Credential Definition Id and Credential Definition json.
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
    logger.debug("parse_get_cred_def_response: >>> get_cred_def_response: %r", get_cred_def_response)

    if not hasattr(parse_get_cred_def_response, "cb"):
        logger.debug("parse_get_cred_def_response: Creating callback")
        parse_get_cred_def_response.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p, c_char_p))

    c_get_cred_def_response = c_char_p(get_cred_def_response.encode('utf-8'))

    (cred_def_id, cred_def_json) = await do_call('indy_parse_get_cred_def_response',
                                                 c_get_cred_def_response,
                                                 parse_get_cred_def_response.cb)

    res = (cred_def_id.decode(), cred_def_json.decode())
    logger.debug("parse_get_cred_def_response: <<< res: %r", res)
    return res


async def build_node_request(submitter_did: str,
                             target_did: str,
                             data: str) -> str:
    """
    Builds a NODE request. Request to add a new node to the pool, or updates existing in the pool.

    :param submitter_did: DID of the submitter stored in secured Wallet.
    :param target_did: Target Node's DID.  It differs from submitter_did field.
    :param data: Data associated with the Node:
      {
          alias: string - Node's alias
          blskey: string - (Optional) BLS multi-signature key as base58-encoded string.
          blskey_pop: string - (Optional) BLS key proof of possession as base58-encoded string.
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


async def build_get_validator_info_request(submitter_did: str) -> str:
    """
    Builds a GET_VALIDATOR_INFO request.
    :param submitter_did: Id of Identity stored in secured Wallet.
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_get_validator_info_request: >>> submitter_did: %r", submitter_did)

    if not hasattr(build_get_validator_info_request, "cb"):
        logger.debug("build_get_validator_info_request: Creating callback")
        build_get_validator_info_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))

    request_json = await do_call('indy_build_get_validator_info_request',
                                 c_submitter_did,
                                 build_get_validator_info_request.cb)

    res = request_json.decode()
    logger.debug("build_get_validator_info_request: <<< res: %r", res)
    return res


async def build_get_txn_request(submitter_did: str,
                                ledger_type: Optional[str],
                                seq_no: int) -> str:
    """
    Builds a GET_TXN request. Request to get any transaction by its seq_no.

    :param submitter_did: DID of the submitter stored in secured Wallet.
    :param ledger_type: (Optional) type of the ledger the requested transaction belongs to:
        DOMAIN - used default,
        POOL,
        CONFIG
        any number
    :param seq_no: requested transaction sequence number as it's stored on Ledger.
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_get_txn_request: >>> submitter_did: %r, ledger_type: %r, seq_no: %r",
                 submitter_did,
                 ledger_type,
                 seq_no)

    if not hasattr(build_get_txn_request, "cb"):
        logger.debug("build_get_txn_request: Creating callback")
        build_get_txn_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_ledger_type = c_char_p(ledger_type.encode('utf-8')) if ledger_type is not None else None
    c_seq_no = c_int32(seq_no)

    request_json = await do_call('indy_build_get_txn_request',
                                 c_submitter_did,
                                 c_ledger_type,
                                 c_seq_no,
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


async def build_pool_restart_request(submitter_did: str, action: str, datetime: str) -> str:
    """
    Builds a POOL_RESTART request

    :param submitter_did: Id of Identity that sender transaction
    :param action       : Action that pool has to do after received transaction.
                          Can be "start" or "cancel"
    :param datetime           : Time when pool must be restarted.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_pool_restart_request: >>> submitter_did: %r, action: %r, datetime: %r")

    if not hasattr(build_pool_restart_request, "cb"):
        logger.debug("build_pool_restart_request: Creating callback")
        build_pool_restart_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_action = c_char_p(action.encode('utf-8'))
    c_datetime = c_char_p(datetime.encode('utf-8')) if datetime else None

    request_json = await do_call('indy_build_pool_restart_request',
                                 c_submitter_did,
                                 c_action,
                                 c_datetime,
                                 build_pool_restart_request.cb)

    res = request_json.decode()
    logger.debug("build_pool_upgrade_request: <<< res: %r", res)
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
    c_schedule = c_char_p(schedule.encode('utf-8')) if schedule is not None else None
    c_justification = c_char_p(justification.encode('utf-8')) if justification is not None else None
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
    to an exists credential definition.

    :param submitter_did:DID of the submitter stored in secured Wallet.
    :param data: Revocation Registry data:
      {
          "id": string - ID of the Revocation Registry,
          "revocDefType": string - Revocation Registry type (only CL_ACCUM is supported for now),
          "tag": string - Unique descriptive ID of the Registry,
          "credDefId": string - ID of the corresponding CredentialDefinition,
          "value": Registry-specific data {
              "issuanceType": string - Type of Issuance(ISSUANCE_BY_DEFAULT or ISSUANCE_ON_DEMAND),
              "maxCredNum": number - Maximum number of credentials the Registry can serve.
              "tailsHash": string - Hash of tails.
              "tailsLocation": string - Location of tails file.
              "publicKeys": <public_keys> - Registry's public key.
          },
          "ver": string - version of revocation registry definition json.
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
    :param rev_reg_def_id: ID of Revocation Registry Definition in ledger.

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


async def parse_get_revoc_reg_def_response(get_revoc_ref_def_response: str) -> (str, str):
    """
    Parse a GET_REVOC_REG_DEF response to get Revocation Registry Definition in the format compatible with Anoncreds API.

    :param get_revoc_ref_def_response: response of GET_REVOC_REG_DEF request.
    :return: Revocation Registry Definition Id and Revocation Registry Definition json.
      {
          "id": string - ID of the Revocation Registry,
          "revocDefType": string - Revocation Registry type (only CL_ACCUM is supported for now),
          "tag": string - Unique descriptive ID of the Registry,
          "credDefId": string - ID of the corresponding CredentialDefinition,
          "value": Registry-specific data {
              "issuanceType": string - Type of Issuance(ISSUANCE_BY_DEFAULT or ISSUANCE_ON_DEMAND),
              "maxCredNum": number - Maximum number of credentials the Registry can serve.
              "tailsHash": string - Hash of tails.
              "tailsLocation": string - Location of tails file.
              "publicKeys": <public_keys> - Registry's public key.
          },
          "ver": string - version of revocation registry definition json.
      }
    """

    logger = logging.getLogger(__name__)
    logger.debug("parse_get_revoc_reg_def_response: >>> get_revoc_ref_def_response: %r", get_revoc_ref_def_response)

    if not hasattr(parse_get_revoc_reg_def_response, "cb"):
        logger.debug("parse_get_revoc_reg_def_response: Creating callback")
        parse_get_revoc_reg_def_response.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p, c_char_p))

    c_get_revoc_ref_def_response = c_char_p(get_revoc_ref_def_response.encode('utf-8'))

    (revoc_reg_def_id, revoc_reg_def_json) = await do_call('indy_parse_get_revoc_reg_def_response',
                                                           c_get_revoc_ref_def_response,
                                                           parse_get_revoc_reg_def_response.cb)

    res = (revoc_reg_def_id.decode(), revoc_reg_def_json.decode())
    logger.debug("parse_get_revoc_reg_def_response: <<< res: %r", res)
    return res


async def build_revoc_reg_entry_request(submitter_did: str,
                                        revoc_reg_def_id: str,
                                        rev_def_type: str,
                                        value: str) -> str:
    """
    Builds a REVOC_REG_ENTRY request.  Request to add the RevocReg entry containing
    the new accumulator value and issued/revoked indices.
    This is just a delta of indices, not the whole list. So, it can be sent each time a new credential is issued/revoked.

    :param submitter_did: DID of the submitter stored in secured Wallet.
    :param revoc_reg_def_id:  ID of the corresponding RevocRegDef.
    :param rev_def_type:  Revocation Registry type (only CL_ACCUM is supported for now).
    :param value: Registry-specific data: 
       {
           value: {
               prevAccum: string - previous accumulator value.
               accum: string - current accumulator value.
               issued: array<number> - an array of issued indices.
               revoked: array<number> an array of revoked indices.
           },
           ver: string - version revocation registry entry json
      
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
    :param revoc_reg_def_id:  ID of the corresponding Revocation Registry Definition in ledger.
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


async def parse_get_revoc_reg_response(get_revoc_reg_response: str) -> (str, str, int):
    """
    Parse a GET_REVOC_REG response to get Revocation Registry in the format compatible with Anoncreds API.

    :param get_revoc_reg_response: response of GET_REVOC_REG request.
    :return: Revocation Registry Definition Id, Revocation Registry json and Timestamp.
      {
          "value": Registry-specific data {
              "accum": string - current accumulator value.
          },
          "ver": string - version revocation registry json
      }
    """

    logger = logging.getLogger(__name__)
    logger.debug("parse_get_revoc_reg_response: >>> get_revoc_reg_response: %r", get_revoc_reg_response)

    if not hasattr(parse_get_revoc_reg_response, "cb"):
        logger.debug("parse_get_revoc_reg_response: Creating callback")
        parse_get_revoc_reg_response.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p, c_char_p, c_uint64))

    c_get_revoc_reg_response = c_char_p(get_revoc_reg_response.encode('utf-8'))

    (revoc_reg_def_id, revoc_reg_json, timestamp) = await do_call('indy_parse_get_revoc_reg_response',
                                                                  c_get_revoc_reg_response,
                                                                  parse_get_revoc_reg_response.cb)

    res = (revoc_reg_def_id.decode(), revoc_reg_json.decode(), timestamp)
    logger.debug("parse_get_revoc_reg_response: <<< res: %r", res)
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
    :param revoc_reg_def_id:  ID of the corresponding Revocation Registry Definition in ledger.
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


async def parse_get_revoc_reg_delta_response(get_revoc_reg_delta_response: str) -> (str, str, int):
    """
    Parse a GET_REVOC_REG_DELTA response to get Revocation Registry Delta in the format compatible with Anoncreds API.

    :param get_revoc_reg_delta_response: response of GET_REVOC_REG_DELTA request.
    :return: Revocation Registry Definition Id, Revocation Registry Delta json and Timestamp.
      {
          "value": Registry-specific data {
              prevAccum: string - previous accumulator value.
              accum: string - current accumulator value.
              issued: array<number> - an array of issued indices.
              revoked: array<number> an array of revoked indices.
          },
          "ver": string
      }
    """

    logger = logging.getLogger(__name__)
    logger.debug("parse_get_revoc_reg_delta_response: >>> get_revoc_reg_delta_response: %r",
                 get_revoc_reg_delta_response)

    if not hasattr(parse_get_revoc_reg_delta_response, "cb"):
        logger.debug("parse_get_revoc_reg_delta_response: Creating callback")
        parse_get_revoc_reg_delta_response.cb = create_cb(
            CFUNCTYPE(None, c_int32, c_int32, c_char_p, c_char_p, c_uint64))

    c_get_revoc_reg_delta_response = c_char_p(get_revoc_reg_delta_response.encode('utf-8'))

    (revoc_reg_def_id, revoc_reg_delta_json, timestamp) = await do_call('indy_parse_get_revoc_reg_delta_response',
                                                                        c_get_revoc_reg_delta_response,
                                                                        parse_get_revoc_reg_delta_response.cb)

    res = (revoc_reg_def_id.decode(), revoc_reg_delta_json.decode(), timestamp)
    logger.debug("parse_get_revoc_reg_delta_response: <<< res: %r", res)
    return res
