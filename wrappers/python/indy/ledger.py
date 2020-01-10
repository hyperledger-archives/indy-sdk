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


async def submit_action(pool_handle: int,
                        request_json: str,
                        nodes: Optional[str],
                        timeout: Optional[int]) -> str:
    """
    Send action to particular nodes of validator pool.

    The list of requests can be send:
        POOL_RESTART
        GET_VALIDATOR_INFO

    The request is sent to the nodes as is. It's assumed that it's already prepared.

    :param pool_handle: pool handle (created by open_pool_ledger).
    :param request_json: Request data json.
    :param nodes: (Optional) List of node names to send the request.
           ["Node1", "Node2",...."NodeN"]
    :param timeout: (Optional) Time to wait respond from nodes (override the default timeout) (in sec).
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("submit_action: >>> pool_handle: %r, request_json: %r, nodes: %r, timeout: %r",
                 pool_handle,
                 request_json,
                 nodes,
                 timeout)

    if not hasattr(submit_action, "cb"):
        logger.debug("submit_action: Creating callback")
        submit_action.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_pool_handle = c_int32(pool_handle)
    c_request_json = c_char_p(request_json.encode('utf-8'))
    c_nodes = c_char_p(nodes.encode('utf-8')) if nodes is not None else None
    c_timeout = c_int32(timeout) if timeout is not None else None

    request_result = await do_call('indy_submit_action',
                                   c_pool_handle,
                                   c_request_json,
                                   c_nodes,
                                   c_timeout,
                                   submit_action.cb)

    res = request_result.decode()
    logger.debug("submit_action: <<< res: %r", res)
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


async def build_get_ddo_request(submitter_did: Optional[str],
                                target_did: str) -> str:
    """
    Builds a request to get a DDO.

    :param submitter_did: (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
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

    c_submitter_did = c_char_p(submitter_did.encode('utf-8')) if submitter_did is not None else None
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

    :param submitter_did: Identifier (DID) of the transaction author as base58-encoded string.
                          Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
    :param target_did: Target DID as base58-encoded string for 16 or 32 bit DID value.
    :param ver_key: Target identity verification key as base58-encoded string.
    :param alias: NYM's alias.
    :param role: Role of a user NYM record:
                             null (common USER)
                             TRUSTEE
                             STEWARD
                             TRUST_ANCHOR
                             ENDORSER - equal to TRUST_ANCHOR that will be removed soon
                             NETWORK_MONITOR
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

    :param submitter_did: Identifier (DID) of the transaction author as base58-encoded string.
                          Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
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


async def build_get_attrib_request(submitter_did: Optional[str],
                                   target_did: str,
                                   raw: Optional[str],
                                   xhash: Optional[str],
                                   enc: Optional[str]) -> str:
    """
    Builds a GET_ATTRIB request. Request to get information about an Attribute for the specified DID.

    :param submitter_did: (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
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

    c_submitter_did = c_char_p(submitter_did.encode('utf-8')) if submitter_did is not None else None
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


async def build_get_nym_request(submitter_did: Optional[str],
                                target_did: str) -> str:
    """
    Builds a GET_NYM request. Request to get information about a DID (NYM).

    :param submitter_did: (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
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

    c_submitter_did = c_char_p(submitter_did.encode('utf-8')) if submitter_did is not None else None
    c_target_did = c_char_p(target_did.encode('utf-8'))

    request_json = await do_call('indy_build_get_nym_request',
                                 c_submitter_did,
                                 c_target_did,
                                 build_get_nym_request.cb)

    res = request_json.decode()
    logger.debug("build_get_nym_request: <<< res: %r", res)
    return res


async def parse_get_nym_response(response: str) -> str:
    """
    Parse a GET_NYM response to get NYM data.

    :param response: response on GET_NYM request.
    :return: NYM data
    {
        did: DID as base58-encoded string for 16 or 32 bit DID value.
        verkey: verification key as base58-encoded string.
        role: Role associated number
                                null (common USER)
                                0 - TRUSTEE
                                2 - STEWARD
                                101 - TRUST_ANCHOR
                                101 - ENDORSER - equal to TRUST_ANCHOR that will be removed soon
                                201 - NETWORK_MONITOR
    }
    """

    logger = logging.getLogger(__name__)
    logger.debug("parse_get_nym_response: >>> response: %r",
                 response)

    if not hasattr(parse_get_nym_response, "cb"):
        logger.debug("parse_get_nym_response: Creating callback")
        parse_get_nym_response.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_response = c_char_p(response.encode('utf-8'))

    request_json = await do_call('indy_parse_get_nym_response',
                                 c_response,
                                 parse_get_nym_response.cb)

    res = request_json.decode()
    logger.debug("parse_get_nym_response: <<< res: %r", res)
    return res


async def build_schema_request(submitter_did: str,
                               data: str) -> str:
    """
    Builds a SCHEMA request. Request to add Credential's schema.

    :param submitter_did: Identifier (DID) of the transaction author as base58-encoded string.
                          Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
    :param data: Credential schema.
                 {
                     id: identifier of schema
                     attrNames: array of attribute name strings (the number of attributes should be less or equal than 125)
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


async def build_get_schema_request(submitter_did: Optional[str],
                                   id_: str) -> str:
    """
    Builds a GET_SCHEMA request. Request to get Credential's Schema.

    :param submitter_did: (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
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

    c_submitter_did = c_char_p(submitter_did.encode('utf-8')) if submitter_did is not None else None
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

    :param submitter_did: Identifier (DID) of the transaction author as base58-encoded string.
                          Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
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


async def build_get_cred_def_request(submitter_did: Optional[str],
                                     id_: str) -> str:
    """
   Builds a GET_CRED_DEF request. Request to get a credential definition (in particular, public key),
   that Issuer creates for a particular Credential Schema.

    :param submitter_did: (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
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

    c_submitter_did = c_char_p(submitter_did.encode('utf-8')) if submitter_did is not None else None
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

    :param submitter_did: Identifier (DID) of the transaction author as base58-encoded string.
                          Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
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


async def build_get_txn_request(submitter_did: Optional[str],
                                ledger_type: Optional[str],
                                seq_no: int) -> str:
    """
    Builds a GET_TXN request. Request to get any transaction by its seq_no.

    :param submitter_did: (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
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

    c_submitter_did = c_char_p(submitter_did.encode('utf-8')) if submitter_did is not None else None
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

    :param submitter_did: Identifier (DID) of the transaction author as base58-encoded string.
                          Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
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

    :param submitter_did: Identifier (DID) of the transaction author as base58-encoded string.
                          Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
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
                                     force: bool,
                                     package: Optional[str]) -> str:
    """
    Builds a POOL_UPGRADE request. Request to upgrade the Pool (sent by Trustee).
    It upgrades the specified Nodes (either all nodes in the Pool, or some specific ones).

    :param submitter_did: Identifier (DID) of the transaction author as base58-encoded string.
                          Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
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
    :param package: (Optional) Package to be upgraded.
    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_pool_upgrade_request: >>> submitter_did: %r, name: %r, version: %r, action: %r, _sha256: %r, "
                 "timeout: %r, schedule: %r, justification: %r, reinstall: %r, force: %r, package: %r",
                 submitter_did, name, version, action, _sha256, _timeout, schedule, justification, reinstall, force,
                 package)

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
    c_package = c_char_p(package.encode('utf-8')) if package is not None else None

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
                                 c_package,
                                 build_pool_upgrade_request.cb)

    res = request_json.decode()
    logger.debug("build_pool_upgrade_request: <<< res: %r", res)
    return res


async def build_revoc_reg_def_request(submitter_did: str,
                                      data: str) -> str:
    """
    Builds a REVOC_REG_DEF request. Request to add the definition of revocation registry
    to an exists credential definition.

    :param submitter_did: Identifier (DID) of the transaction author as base58-encoded string.
                          Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
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


async def build_get_revoc_reg_def_request(submitter_did: Optional[str],
                                          rev_reg_def_id: str) -> str:
    """
    Builds a GET_REVOC_REG_DEF request. Request to get a revocation registry definition,
    that Issuer creates for a particular Credential Definition.

    :param submitter_did: (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
    :param rev_reg_def_id: ID of Revocation Registry Definition in ledger.

    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_get_revoc_reg_def_request: >>> submitter_did: %r, rev_reg_def_id: %r", submitter_did,
                 rev_reg_def_id)

    if not hasattr(build_get_revoc_reg_def_request, "cb"):
        logger.debug("build_get_revoc_reg_def_request: Creating callback")
        build_get_revoc_reg_def_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8')) if submitter_did is not None else None
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

    :param submitter_did: Identifier (DID) of the transaction author as base58-encoded string.
                          Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
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


async def build_get_revoc_reg_request(submitter_did: Optional[str],
                                      revoc_reg_def_id: str,
                                      timestamp: int) -> str:
    """
    Builds a GET_REVOC_REG request. Request to get the accumulated state of the Revocation Registry
    by ID. The state is defined by the given timestamp.

    :param submitter_did: (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
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

    c_submitter_did = c_char_p(submitter_did.encode('utf-8')) if submitter_did is not None else None
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


async def build_get_revoc_reg_delta_request(submitter_did: Optional[str],
                                            revoc_reg_def_id: str,
                                            from_: Optional[int],
                                            to: int) -> str:
    """
    Builds a GET_REVOC_REG_DELTA request. Request to get the delta of the accumulated state of the Revocation Registry.
    The Delta is defined by from and to timestamp fields.
    If from is not specified, then the whole state till to will be returned.

    :param submitter_did: (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
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

    c_submitter_did = c_char_p(submitter_did.encode('utf-8')) if submitter_did is not None else None
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


async def get_response_metadata(response: str) -> str:
    """
     Parse transaction response to fetch metadata.
     The important use case for this method is validation of Node's response freshens.

     Distributed Ledgers can reply with outdated information for consequence read request after write.
     To reduce pool load libindy sends read requests to one random node in the pool.
     Consensus validation is performed based on validation of nodes multi signature for current ledger Merkle Trie root.
     This multi signature contains information about the latest ldeger's transaction ordering time and sequence number that this method returns.

     If node that returned response for some reason is out of consensus and has outdated ledger
     it can be caught by analysis of the returned latest ledger's transaction ordering time and sequence number.

     There are two ways to filter outdated responses:
         1) based on "seqNo" - sender knows the sequence number of transaction that he consider as a fresh enough.
         2) based on "txnTime" - sender knows the timestamp that he consider as a fresh enough.

     Note: response of GET_VALIDATOR_INFO request isn't supported

    :param response: response of write or get request.
    :return: Response Metadata.
    {
        "seqNo": Option<u64> - transaction sequence number,
        "txnTime": Option<u64> - transaction ordering time,
        "lastSeqNo": Option<u64> - the latest transaction seqNo for particular Node,
        "lastTxnTime": Option<u64> - the latest transaction ordering time for particular Node
    }
    """

    logger = logging.getLogger(__name__)
    logger.debug("get_response_metadata: >>> response: %r",
                 response)

    if not hasattr(get_response_metadata, "cb"):
        logger.debug("get_response_metadata: Creating callback")
        get_response_metadata.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_response = c_char_p(response.encode('utf-8'))

    response_metadata = await do_call('indy_get_response_metadata',
                                      c_response,
                                      get_response_metadata.cb)

    res = response_metadata.decode()
    logger.debug("get_response_metadata: <<< res: %r", res)
    return res


async def build_auth_rule_request(submitter_did: str,
                                  txn_type: str,
                                  action: str,
                                  field: str,
                                  old_value: Optional[str],
                                  new_value: Optional[str],
                                  constraint: str) -> str:
    """
    Builds a AUTH_RULE request. Request to change authentication rules for a ledger transaction.

    :param submitter_did: Identifier (DID) of the transaction author as base58-encoded string.
                          Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
    :param txn_type: ledger transaction alias or associated value.
    :param action: type of an action.
       Can be either "ADD" (to add a new rule) or "EDIT" (to edit an existing one).
    :param field: transaction field.
    :param old_value: (Optional) old value of a field, which can be changed to a new_value (mandatory for EDIT action).
    :param new_value: (Optional) new value that can be used to fill the field.
    :param constraint: set of constraints required for execution of an action in the following format:
        {
            constraint_id - <string> type of a constraint.
                Can be either "ROLE" to specify final constraint or  "AND"/"OR" to combine constraints.
            role - <string> (optional) role of a user which satisfy to constrain.
            sig_count - <u32> the number of signatures required to execution action.
            need_to_be_owner - <bool> (optional) if user must be an owner of transaction (false by default).
            off_ledger_signature - <bool> (optional) allow signature of unknow for ledger did (false by default).
            metadata - <object> (optional) additional parameters of the constraint.
        }
      can be combined by
        {
            'constraint_id': <"AND" or "OR">
            'auth_constraints': [<constraint_1>, <constraint_2>]
        }

    Default ledger auth rules: https://github.com/hyperledger/indy-node/blob/master/docs/source/auth_rules.md

    More about AUTH_RULE request: https://github.com/hyperledger/indy-node/blob/master/docs/source/requests.md#auth_rule

    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_auth_rule_request: >>> submitter_did: %r, txn_type: %r, action: %r, field: %r, "
                 "old_value: %r, new_value: %r, constraint: %r",
                 submitter_did,
                 txn_type,
                 action,
                 field,
                 old_value,
                 new_value,
                 constraint)

    if not hasattr(build_auth_rule_request, "cb"):
        logger.debug("build_auth_rule_request: Creating callback")
        build_auth_rule_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_txn_type = c_char_p(txn_type.encode('utf-8'))
    c_action = c_char_p(action.encode('utf-8'))
    c_field = c_char_p(field.encode('utf-8'))
    c_old_value = c_char_p(old_value.encode('utf-8')) if old_value is not None else None
    c_new_value = c_char_p(new_value.encode('utf-8')) if new_value is not None else None
    c_constraint = c_char_p(constraint.encode('utf-8'))

    request_json = await do_call('indy_build_auth_rule_request',
                                 c_submitter_did,
                                 c_txn_type,
                                 c_action,
                                 c_field,
                                 c_old_value,
                                 c_new_value,
                                 c_constraint,
                                 build_auth_rule_request.cb)

    res = request_json.decode()
    logger.debug("build_auth_rule_request: <<< res: %r", res)
    return res


async def build_auth_rules_request(submitter_did: str,
                                   data: str) -> str:
    """
    Builds a AUTH_RULES request. Request to change multiple authentication rules for a ledger transaction.

    :param submitter_did: Identifier (DID) of the transaction author as base58-encoded string.
                          Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
    :param data: a list of auth rules: [
        {
            "auth_type": ledger transaction alias or associated value,
            "auth_action": type of an action,
            "field": transaction field,
            "old_value": (Optional) old value of a field, which can be changed to a new_value (mandatory for EDIT action),
            "new_value": (Optional) new value that can be used to fill the field,
            "constraint": set of constraints required for execution of an action in the format described above for `build_auth_rule_request` function.
        }
    ]

    Default ledger auth rules: https://github.com/hyperledger/indy-node/blob/master/docs/source/auth_rules.md

    More about AUTH_RULE request: https://github.com/hyperledger/indy-node/blob/master/docs/source/requests.md#auth_rules

    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_auth_rules_request: >>> submitter_did: %r, data: %r",
                 submitter_did,
                 data)

    if not hasattr(build_auth_rules_request, "cb"):
        logger.debug("build_auth_rules_request: Creating callback")
        build_auth_rules_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_data = c_char_p(data.encode('utf-8'))

    request_json = await do_call('indy_build_auth_rules_request',
                                 c_submitter_did,
                                 c_data,
                                 build_auth_rules_request.cb)

    res = request_json.decode()
    logger.debug("build_auth_rules_request: <<< res: %r", res)
    return res


async def build_get_auth_rule_request(submitter_did: Optional[str],
                                      txn_type: Optional[str],
                                      action: Optional[str],
                                      field: Optional[str],
                                      old_value: Optional[str],
                                      new_value: Optional[str]) -> str:
    """
    Builds a GET_AUTH_RULE request. Request to get authentication rules for a ledger transaction.

    NOTE: Either none or all transaction related parameters must be specified (`old_value` can be skipped for `ADD` action).
        * none - to get all authentication rules for all ledger transactions
        * all - to get authentication rules for specific action (`old_value` can be skipped for `ADD` action)

    :param submitter_did: (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
    :param txn_type: target ledger transaction alias or associated value.
    :param action: target action type. Can be either "ADD" or "EDIT".
    :param field: target transaction field.
    :param old_value: (Optional) old value of field, which can be changed to a new_value (must be specified for EDIT action).
    :param new_value: (Optional) new value that can be used to fill the field.

    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_get_auth_rule_request: >>> submitter_did: %r, txn_type: %r, action: %r, field: %r, "
                 "old_value: %r, new_value: %r",
                 submitter_did,
                 txn_type,
                 action,
                 field,
                 old_value,
                 new_value)

    if not hasattr(build_get_auth_rule_request, "cb"):
        logger.debug("build_get_auth_rule_request: Creating callback")
        build_get_auth_rule_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8')) if submitter_did is not None else None
    c_txn_type = c_char_p(txn_type.encode('utf-8')) if txn_type is not None else None
    c_action = c_char_p(action.encode('utf-8')) if action is not None else None
    c_field = c_char_p(field.encode('utf-8')) if field is not None else None
    c_old_value = c_char_p(old_value.encode('utf-8')) if old_value is not None else None
    c_new_value = c_char_p(new_value.encode('utf-8')) if new_value is not None else None

    request_json = await do_call('indy_build_get_auth_rule_request',
                                 c_submitter_did,
                                 c_txn_type,
                                 c_action,
                                 c_field,
                                 c_old_value,
                                 c_new_value,
                                 build_get_auth_rule_request.cb)

    res = request_json.decode()
    logger.debug("build_get_auth_rule_request: <<< res: %r", res)
    return res


async def build_txn_author_agreement_request(submitter_did: str,
                                             text: Optional[str],
                                             version: str,
                                             ratification_ts: Optional[int] = None,
                                             retirement_ts: Optional[int] = None) -> str:
    """
    Builds a TXN_AUTHR_AGRMT request. Request to add a new version of Transaction Author Agreement to the ledger.

    EXPERIMENTAL

    :param submitter_did: Identifier (DID) of the transaction author as base58-encoded string.
                          Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
    :param text: (Optional) a content of the TTA.
                          Mandatory in case of adding a new TAA. An existing TAA text can not be changed.
                          for Indy Node version <= 1.12.0:
                              Use empty string to reset TAA on the ledger
                          for Indy Node version > 1.12.0
                              Should be omitted in case of updating an existing TAA (setting `retirement_ts`)
    :param version: a version of the TTA (unique UTF-8 string).
    :param ratification_ts: Optional) the date (timestamp) of TAA ratification by network government.
                          for Indy Node version <= 1.12.0:
                             Must be omitted
                          for Indy Node version > 1.12.0:
                             Must be specified in case of adding a new TAA
                             Can be omitted in case of updating an existing TAA
    :param retirement_ts: (Optional) the date (timestamp) of TAA retirement.
                          for Indy Node version <= 1.12.0:
                              Must be omitted
                          for Indy Node version > 1.12.0:
                              Must be omitted in case of adding a new (latest) TAA.
                              Should be used for updating (deactivating) non-latest TAA on the ledger.

    Note: Use `build_disable_all_txn_author_agreements_request` to disable all TAA's on the ledger.

    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_txn_author_agreement_request: >>> submitter_did: %r, text: %r, version: %r, "
                 "ratification_ts: %r, retirement_ts: %r",
                 submitter_did,
                 text,
                 version,
                 ratification_ts,
                 retirement_ts)

    if not hasattr(build_txn_author_agreement_request, "cb"):
        logger.debug("build_txn_author_agreement_request: Creating callback")
        build_txn_author_agreement_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_text = c_char_p(text.encode('utf-8')) if text is not None else None
    c_version = c_char_p(version.encode('utf-8'))
    c_ratification_ts = c_int64(ratification_ts) if ratification_ts is not None else c_int(-1)
    c_retirement_ts = c_int64(retirement_ts) if retirement_ts is not None else c_int(-1)

    request_json = await do_call('indy_build_txn_author_agreement_request',
                                 c_submitter_did,
                                 c_text,
                                 c_version,
                                 c_ratification_ts,
                                 c_retirement_ts,
                                 build_txn_author_agreement_request.cb)

    res = request_json.decode()
    logger.debug("build_txn_author_agreement_request: <<< res: %r", res)
    return res


async def build_disable_all_txn_author_agreements_request(submitter_did: str) -> str:
    """
    Builds a DISABLE_ALL_TXN_AUTHR_AGRMTS request. Request to disable all Transaction Author Agreement on the ledger.

    EXPERIMENTAL

    :param submitter_did: Identifier (DID) of the transaction author as base58-encoded string.
                          Actual request sender may differ if Endorser is used (look at `append_request_endorser`)

    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_disable_all_txn_author_agreements_request: >>> submitter_did: %r",
                 submitter_did)

    if not hasattr(build_disable_all_txn_author_agreements_request, "cb"):
        logger.debug("build_disable_all_txn_author_agreements_request: Creating callback")
        build_disable_all_txn_author_agreements_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))

    request_json = await do_call('indy_build_disable_all_txn_author_agreements_request',
                                 c_submitter_did,
                                 build_disable_all_txn_author_agreements_request.cb)

    res = request_json.decode()
    logger.debug("build_disable_all_txn_author_agreements_request: <<< res: %r", res)
    return res


async def build_get_txn_author_agreement_request(submitter_did: Optional[str],
                                                 data: Optional[str]) -> str:
    """
    Builds a GET_TXN_AUTHR_AGRMT request. Request to get a specific Transaction Author Agreement from the ledger.

    EXPERIMENTAL

    :param submitter_did: (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
    :param data: (Optional) specifies a condition for getting specific TAA.
     Contains 3 mutually exclusive optional fields:
     {
         hash: Optional<str> - hash of requested TAA,
         version: Optional<str> - version of requested TAA.
         timestamp: Optional<i64> - ledger will return TAA valid at requested timestamp.
     }
     Null data or empty JSON are acceptable here. In this case, ledger will return the latest version of TAA.

    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_get_txn_author_agreement_request: >>> submitter_did: %r, data: %r",
                 submitter_did,
                 data)

    if not hasattr(build_get_txn_author_agreement_request, "cb"):
        logger.debug("build_get_txn_author_agreement_request: Creating callback")
        build_get_txn_author_agreement_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8')) if submitter_did is not None else None
    c_data = c_char_p(data.encode('utf-8')) if data is not None else None

    request_json = await do_call('indy_build_get_txn_author_agreement_request',
                                 c_submitter_did,
                                 c_data,
                                 build_get_txn_author_agreement_request.cb)

    res = request_json.decode()
    logger.debug("build_get_txn_author_agreement_request: <<< res: %r", res)
    return res


async def build_acceptance_mechanisms_request(submitter_did: str,
                                              aml: str,
                                              version: str,
                                              aml_context: Optional[str]) -> str:
    """
    Builds a SET_TXN_AUTHR_AGRMT_AML request. Request to add a new list of acceptance mechanisms for transaction author agreement.
    Acceptance Mechanism is a description of the ways how the user may accept a transaction author agreement.

    EXPERIMENTAL

    :param submitter_did: Identifier (DID) of the transaction author as base58-encoded string.
                          Actual request sender may differ if Endorser is used (look at `append_request_endorser`)
    :param aml: a set of new acceptance mechanisms:
    {
        “<acceptance mechanism label 1>”: { acceptance mechanism description 1},
        “<acceptance mechanism label 2>”: { acceptance mechanism description 2},
        ...
    }
    :param version: a version of new acceptance mechanisms. (Note: unique on the Ledger)
    :param aml_context: (Optional) common context information about acceptance mechanisms (may be a URL to external resource).

    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_acceptance_mechanisms_request: >>> submitter_did: %r, aml: %r, version: %r, aml_context: %r",
                 submitter_did,
                 aml,
                 version,
                 aml_context)

    if not hasattr(build_acceptance_mechanisms_request, "cb"):
        logger.debug("build_acceptance_mechanisms_request: Creating callback")
        build_acceptance_mechanisms_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8'))
    c_aml = c_char_p(aml.encode('utf-8'))
    c_version = c_char_p(version.encode('utf-8'))
    c_aml_context = c_char_p(aml_context.encode('utf-8')) if aml_context is not None else None

    request_json = await do_call('indy_build_acceptance_mechanisms_request',
                                 c_submitter_did,
                                 c_aml,
                                 c_version,
                                 c_aml_context,
                                 build_acceptance_mechanisms_request.cb)

    res = request_json.decode()
    logger.debug("build_acceptance_mechanisms_request: <<< res: %r", res)
    return res


async def build_get_acceptance_mechanisms_request(submitter_did: Optional[str],
                                                  timestamp: Optional[int],
                                                  version: Optional[str]) -> str:
    """
    Builds a GET_TXN_AUTHR_AGRMT_AML request. Request to get a list of  acceptance mechanisms from the ledger
    valid for specified time or the latest one.

    EXPERIMENTAL

    :param submitter_did: (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
    :param timestamp: (Optional) time to get an active acceptance mechanisms. The latest one will be returned for the empty timestamp.
    :param version: (Optional) version of acceptance mechanisms.

    NOTE: timestamp and version cannot be specified together.

    :return: Request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_get_acceptance_mechanisms_request: >>> submitter_did: %r, timestamp: %r, version: %r",
                 submitter_did,
                 timestamp,
                 version)

    if not hasattr(build_get_acceptance_mechanisms_request, "cb"):
        logger.debug("build_get_acceptance_mechanisms_request: Creating callback")
        build_get_acceptance_mechanisms_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_submitter_did = c_char_p(submitter_did.encode('utf-8')) if submitter_did is not None else None
    c_timestamp = c_int64(timestamp) if timestamp is not None else c_int(-1)
    c_version = c_char_p(version.encode('utf-8')) if version is not None else None

    request_json = await do_call('indy_build_get_acceptance_mechanisms_request',
                                 c_submitter_did,
                                 c_timestamp,
                                 c_version,
                                 build_get_acceptance_mechanisms_request.cb)

    res = request_json.decode()
    logger.debug("build_get_acceptance_mechanisms_request: <<< res: %r", res)
    return res


async def append_txn_author_agreement_acceptance_to_request(request_json: str,
                                                            text: Optional[str],
                                                            version: Optional[str],
                                                            taa_digest: Optional[str],
                                                            mechanism: str,
                                                            time: int) -> str:
    """
    Append transaction author agreement acceptance data to a request.
    This function should be called before signing and sending a request
    if there is any transaction author agreement set on the Ledger.

    EXPERIMENTAL

    This function may calculate hash by itself or consume it as a parameter.
    If all text, version and taa_digest parameters are specified, a check integrity of them will be done.

    :param request_json: original request data json.
    :param text and version: (Optional) raw data about TAA from ledger.
               These parameters should be passed together.
               These parameters are required if taa_digest parameter is omitted.
    :param taa_digest: (Optional) digest on text and version.
                      Digest is sha256 hash calculated on concatenated strings: version || text.
                      This parameter is required if text and version parameters are omitted.
    :param mechanism: mechanism how user has accepted the TAA
    :param time: UTC timestamp when user has accepted the TAA. Note that the time portion will be discarded to avoid a privacy risk.

    :return: Updated request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug(
        "append_txn_author_agreement_acceptance_to_request: >>> request_json: %r, text: %r, version: %r, hash: %r, "
        "acc_mech_type: %r, time_of_acceptance: %r",
        request_json,
        text,
        version,
        taa_digest,
        mechanism,
        time)

    if not hasattr(append_txn_author_agreement_acceptance_to_request, "cb"):
        logger.debug("append_txn_author_agreement_acceptance_to_request: Creating callback")
        append_txn_author_agreement_acceptance_to_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_request_json = c_char_p(request_json.encode('utf-8'))
    c_text = c_char_p(text.encode('utf-8')) if text is not None else None
    c_version = c_char_p(version.encode('utf-8')) if version is not None else None
    c_taa_digest = c_char_p(taa_digest.encode('utf-8')) if taa_digest is not None else None
    c_mechanism = c_char_p(mechanism.encode('utf-8'))

    request_json = await do_call('indy_append_txn_author_agreement_acceptance_to_request',
                                 c_request_json,
                                 c_text,
                                 c_version,
                                 c_taa_digest,
                                 c_mechanism,
                                 c_uint64(time),
                                 append_txn_author_agreement_acceptance_to_request.cb)

    res = request_json.decode()
    logger.debug("append_txn_author_agreement_acceptance_to_request: <<< res: %r", res)
    return res


async def append_request_endorser(request_json: str,
                                  endorser_did: str) -> str:
    """
    Append Endorser to an existing request.

    An author of request still is a `DID` used as a `submitter_did` parameter for the building of the request.
    But it is expecting that the transaction will be sent by the specified Endorser.

    Note: Both Transaction Author and Endorser must sign output request after that.

    More about Transaction Endorser: https://github.com/hyperledger/indy-node/blob/master/design/transaction_endorser.md
                                     https://github.com/hyperledger/indy-sdk/blob/master/docs/configuration.md

    :param request_json: original request data json.
    :param endorser_did: DID of the Endorser that will submit the transaction.
                         The Endorser's DID must be present on the ledger.

    :return: Updated request result as json.
    """

    logger = logging.getLogger(__name__)
    logger.debug(
        "append_request_endorser: >>> request_json: %r, endorser_did: %r",
        request_json,
        endorser_did)

    if not hasattr(append_request_endorser, "cb"):
        logger.debug("append_request_endorser: Creating callback")
        append_request_endorser.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_request_json = c_char_p(request_json.encode('utf-8'))
    c_endorser_did = c_char_p(endorser_did.encode('utf-8'))

    request_json = await do_call('indy_append_request_endorser',
                                 c_request_json,
                                 c_endorser_did,
                                 append_request_endorser.cb)

    res = request_json.decode()
    logger.debug("append_request_endorser: <<< res: %r", res)
    return res
