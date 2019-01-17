from .libindy import do_call, create_cb

from typing import Optional
from ctypes import *

import logging


async def create_payment_address(wallet_handle: int,
                                 payment_method: str,
                                 config: str) -> str:
    """
     Create the payment address for specified payment method


     This method generates private part of payment address
     and stores it in a secure place. Ideally it should be
     secret in libindy wallet (see crypto module).

     Note that payment method should be able to resolve this
     secret by fully resolvable payment address format.

    :param wallet_handle: wallet handle (created by open_wallet).
    :param payment_method: Payment method to use (for example, 'sov').
    :param config: payment address config as json:
       {
         seed: <str>, // allows deterministic creation of payment address
       }
    :return: payment_address: public identifier of payment address in fully resolvable payment address format.
    """

    logger = logging.getLogger(__name__)
    logger.debug("create_payment_address: >>> wallet_handle: %r, payment_method: %r, config: %r",
                 wallet_handle,
                 payment_method,
                 config)

    if not hasattr(create_payment_address, "cb"):
        logger.debug("create_payment_address: Creating callback")
        create_payment_address.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_payment_method = c_char_p(payment_method.encode('utf-8'))
    config = c_char_p(config.encode('utf-8'))

    request_result = await do_call('indy_create_payment_address',
                                   c_wallet_handle,
                                   c_payment_method,
                                   config,
                                   create_payment_address.cb)

    res = request_result.decode()
    logger.debug("create_payment_address: <<< res: %r", res)
    return res


async def list_payment_addresses(wallet_handle: int) -> str:
    """
     Lists all payment addresses that are stored in the wallet

    :param wallet_handle: wallet handle (created by open_wallet).
    :return: payment_addresses_json: json array of string with json addresses
    """

    logger = logging.getLogger(__name__)
    logger.debug("list_payment_addresses: >>> wallet_handle: %r",
                 wallet_handle)

    if not hasattr(list_payment_addresses, "cb"):
        logger.debug("list_payment_addresses: Creating callback")
        list_payment_addresses.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)

    request_result = await do_call('indy_list_payment_addresses',
                                   c_wallet_handle,
                                   list_payment_addresses.cb)

    res = request_result.decode()
    logger.debug("list_payment_addresses: <<< res: %r", res)
    return res


async def add_request_fees(wallet_handle: int,
                           submitter_did: str,
                           req_json: str,
                           inputs_json: str,
                           outputs_json: str,
                           extra: Optional[str]) -> (str, str):
    """
    Modifies Indy request by adding information how to pay fees for this transaction
    according to this payment method.
   
    This method consumes set of inputs and outputs. The difference between inputs balance
    and outputs balance is the fee for this transaction.
   
    Not that this method also produces correct fee signatures.
   
    Format of inputs is specific for payment method. Usually it should reference payment transaction
    with at least one output that corresponds to payment address that user owns.

    :param wallet_handle: wallet handle (created by open_wallet).
    :param submitter_did : (Option) DID of request sender
    :param req_json : initial transaction request as json
    :param inputs_json: The list of payment sources as json array:
      ["source1", ...]
        - each input should reference paymentAddress
        - this param will be used to determine payment_method
    :param outputs_json: The list of outputs as json array:
      [{
        recipient: <str>, // payment address of recipient
        amount: <int>, // amount
      }]
    :param extra: // optional information for payment operation

    :return:
        req_with_fees_json: modified Indy request with added fees info,
        payment_method: used payment method
    """

    logger = logging.getLogger(__name__)
    logger.debug(
        "add_request_fees: >>> wallet_handle: %r, submitter_did: %r, req_json: %r, inputs_json: %r, outputs_json: %r, "
        "extra: %r",
        wallet_handle,
        submitter_did,
        req_json,
        inputs_json,
        outputs_json,
        extra)

    if not hasattr(add_request_fees, "cb"):
        logger.debug("add_request_fees: Creating callback")
        add_request_fees.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_submitter_did = c_char_p(submitter_did.encode('utf-8')) if submitter_did is not None else None
    c_req_json = c_char_p(req_json.encode('utf-8'))
    c_inputs_json = c_char_p(inputs_json.encode('utf-8'))
    c_outputs_json = c_char_p(outputs_json.encode('utf-8'))
    c_extra = c_char_p(extra.encode('utf-8')) if extra is not None else None

    (req_with_fees_json, payment_method) = await do_call('indy_add_request_fees',
                                                         c_wallet_handle,
                                                         c_submitter_did,
                                                         c_req_json,
                                                         c_inputs_json,
                                                         c_outputs_json,
                                                         c_extra,
                                                         add_request_fees.cb)
    res = (req_with_fees_json.decode(), payment_method.decode())

    logger.debug("add_request_fees: <<< res: %r", res)
    return res


async def parse_response_with_fees(payment_method: str,
                                   resp_json: str) -> str:
    """
    Parses response for Indy request with fees.

    :param payment_method: Payment method to use (for example, 'sov').
    :param resp_json: response for Indy request with fees
               Note: this param will be used to determine payment_method
    :return: receipts_json - parsed (payment method and node version agnostic) receipts info as json:
      [{
         receipt: <str>, // receipt that can be used for payment referencing and verification
         recipient: <str>, //payment address of recipient
         amount: <int>, // amount
         extra: <str>, // optional data from payment transaction
      }]
    """

    logger = logging.getLogger(__name__)
    logger.debug("parse_response_with_fees: >>> payment_method: %r, resp_json: %r",
                 payment_method,
                 resp_json)

    if not hasattr(parse_response_with_fees, "cb"):
        logger.debug("parse_response_with_fees: Creating callback")
        parse_response_with_fees.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_payment_method = c_char_p(payment_method.encode('utf-8'))
    c_resp_json = c_char_p(resp_json.encode('utf-8'))

    receipts_json = await do_call('indy_parse_response_with_fees',
                                  c_payment_method,
                                  c_resp_json,
                                  parse_response_with_fees.cb)

    res = receipts_json.decode()
    logger.debug("parse_response_with_fees: <<< res: %r", res)
    return res


async def build_get_payment_sources_request(wallet_handle: int,
                                            submitter_did: str,
                                            payment_address: str) -> (str, str):
    """
    Builds Indy request for getting sources list for payment address
    according to this payment method.

    :param wallet_handle: wallet handle (created by open_wallet).
    :param submitter_did : (Option) DID of request sender
    :param payment_address: target payment address
    :return: get_sources_txn_json: Indy request for getting sources list for payment address
             payment_method: used payment method
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_get_payment_sources_request: >>> wallet_handle: %r, submitter_did: %r, payment_address: %r",
                 wallet_handle,
                 submitter_did,
                 payment_address)

    if not hasattr(build_get_payment_sources_request, "cb"):
        logger.debug("build_get_payment_sources_request: Creating callback")
        build_get_payment_sources_request.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_submitter_did = c_char_p(submitter_did.encode('utf-8')) if submitter_did is not None else None
    c_payment_address = c_char_p(payment_address.encode('utf-8'))

    (get_sources_txn_json, payment_method) = await do_call('indy_build_get_payment_sources_request',
                                                           c_wallet_handle,
                                                           c_submitter_did,
                                                           c_payment_address,
                                                           build_get_payment_sources_request.cb)
    res = (get_sources_txn_json.decode(), payment_method.decode())

    logger.debug("build_get_payment_sources_request: <<< res: %r", res)
    return res


async def parse_get_payment_sources_response(payment_method: str,
                                             resp_json: str) -> str:
    """
    Parses response for Indy request for getting sources list.

    :param payment_method: Payment method to use (for example, 'sov').
    :param resp_json: resp_json: response for Indy request for getting sources list
                      Note: this param will be used to determine payment_method
    :return: sources_json: parsed (payment method and node version agnostic) sources info as json:
      [{
         source: <str>, // source input
         paymentAddress: <str>, //payment address for this source
         amount: <int>, // amount
         extra: <str>, // optional data from payment transaction
      }]
    """

    logger = logging.getLogger(__name__)
    logger.debug("parse_get_payment_sources_response: >>> payment_method: %r, resp_json: %r",
                 payment_method,
                 resp_json)

    if not hasattr(parse_get_payment_sources_response, "cb"):
        logger.debug("parse_get_payment_sources_response: Creating callback")
        parse_get_payment_sources_response.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_payment_method = c_char_p(payment_method.encode('utf-8'))
    c_resp_json = c_char_p(resp_json.encode('utf-8'))

    sources_json = await do_call('indy_parse_get_payment_sources_response',
                                 c_payment_method,
                                 c_resp_json,
                                 parse_get_payment_sources_response.cb)

    res = sources_json.decode()
    logger.debug("parse_get_payment_sources_response: <<< res: %r", res)
    return res


async def build_payment_req(wallet_handle: int,
                            submitter_did: str,
                            inputs_json: str,
                            outputs_json: str,
                            extra: Optional[str]) -> (str, str):
    """
    Builds Indy request for doing payment
    according to this payment method.
   
    This method consumes set of inputs and outputs.
   
    Format of inputs is specific for payment method. Usually it should reference payment transaction
    with at least one output that corresponds to payment address that user owns.

    :param wallet_handle: wallet handle (created by open_wallet).
    :param submitter_did : (Option) DID of request sender
    :param inputs_json: The list of payment sources as json array:
      ["source1", ...]
      Note that each source should reference payment address    
    :param outputs_json: The list of outputs as json array:
      [{
        recipient: <str>, // payment address of recipient
        amount: <int>, // amount
      }]
    :param extra: // optional information for payment operation

    :return: payment_req_json: Indy request for doing payment
             payment_method: used payment method
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_payment_req: >>> wallet_handle: %r, submitter_did: %r, inputs_json: %r, outputs_json: %r,"
                 " extra: %r",
                 wallet_handle,
                 submitter_did,
                 inputs_json,
                 outputs_json,
                 extra)

    if not hasattr(build_payment_req, "cb"):
        logger.debug("build_payment_req: Creating callback")
        build_payment_req.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_submitter_did = c_char_p(submitter_did.encode('utf-8')) if submitter_did is not None else None
    c_inputs_json = c_char_p(inputs_json.encode('utf-8'))
    c_outputs_json = c_char_p(outputs_json.encode('utf-8'))
    c_extra = c_char_p(extra.encode('utf-8')) if extra is not None else None

    (payment_req_json, payment_method) = await do_call('indy_build_payment_req',
                                                       c_wallet_handle,
                                                       c_submitter_did,
                                                       c_inputs_json,
                                                       c_outputs_json,
                                                       c_extra,
                                                       build_payment_req.cb)
    res = (payment_req_json.decode(), payment_method.decode())

    logger.debug("build_payment_req: <<< res: %r", res)
    return res


async def parse_payment_response(payment_method: str,
                                 resp_json: str) -> str:
    """
    Parses response for Indy request for payment txn.

    :param payment_method: Payment method to use (for example, 'sov').
    :param resp_json: resp_json: response for Indy request for payment txn
      Note: this param will be used to determine payment_method
    :return: receipts_json: parsed (payment method and node version agnostic) receipts info as json:
      [{
         receipt: <str>, // receipt that can be used for payment referencing and verification
         recipient: <str>, // payment address of recipient
         amount: <int>, // amount
         extra: <str>, // optional data from payment transaction
      }]
    """

    logger = logging.getLogger(__name__)
    logger.debug("parse_payment_response: >>> wallet_handle: %r, payment_method: %r, resp_json: %r",
                 payment_method,
                 resp_json)

    if not hasattr(parse_payment_response, "cb"):
        logger.debug("parse_payment_response: Creating callback")
        parse_payment_response.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_payment_method = c_char_p(payment_method.encode('utf-8'))
    c_resp_json = c_char_p(resp_json.encode('utf-8'))

    receipts_json = await do_call('indy_parse_payment_response',
                                  c_payment_method,
                                  c_resp_json,
                                  parse_payment_response.cb)

    res = receipts_json.decode()
    logger.debug("parse_payment_response: <<< res: %r", res)
    return res


async def build_mint_req(wallet_handle: int,
                         submitter_did: str,
                         outputs_json: str,
                         extra: Optional[str]) -> (str, str):
    """
    Builds Indy request for doing minting
    according to this payment method.

    :param wallet_handle: wallet handle (created by open_wallet).
    :param submitter_did : (Option) DID of request sender
    :param outputs_json: The list of outputs as json array:
      [{
        recipient: <str>, // payment address of recipient
        amount: <int>, // amount
        extra: <str>, // optional data
      }]
    :param extra: // optional information for payment operation

    :return: mint_req_json: Indy request for doing minting
             payment_method: used payment method
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_mint_req: >>> wallet_handle: %r, submitter_did: %r, outputs_json: %r, extra: %r",
                 wallet_handle,
                 submitter_did,
                 outputs_json,
                 extra)

    if not hasattr(build_mint_req, "cb"):
        logger.debug("build_mint_req: Creating callback")
        build_mint_req.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_submitter_did = c_char_p(submitter_did.encode('utf-8')) if submitter_did is not None else None
    c_outputs_json = c_char_p(outputs_json.encode('utf-8'))
    c_extra = c_char_p(extra.encode('utf-8')) if extra is not None else None

    (mint_req_json, payment_method) = await do_call('indy_build_mint_req',
                                                    c_wallet_handle,
                                                    c_submitter_did,
                                                    c_outputs_json,
                                                    c_extra,
                                                    build_mint_req.cb)
    res = (mint_req_json.decode(), payment_method.decode())

    logger.debug("build_mint_req: <<< res: %r", res)
    return res


async def build_set_txn_fees_req(wallet_handle: int,
                                 submitter_did: str,
                                 payment_method: str,
                                 fees_json: str) -> str:
    """
    Builds Indy request for setting fees for transactions in the ledger

    :param wallet_handle: wallet handle (created by open_wallet).
    :param submitter_did : (Option) DID of request sender
    :param payment_method: Payment method to use (for example, 'sov').
    :param fees_json: {
       txnType1: amount1,
       txnType2: amount2,
       .................
       txnTypeN: amountN,
     }
    :return: set_txn_fees_json: Indy request for setting fees for transactions in the ledger
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_set_txn_fees_req: >>> wallet_handle: %r, submitter_did: %r, payment_method: %r, fees_json: %r",
                 wallet_handle,
                 submitter_did,
                 payment_method,
                 fees_json)

    if not hasattr(build_set_txn_fees_req, "cb"):
        logger.debug("build_set_txn_fees_req: Creating callback")
        build_set_txn_fees_req.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_submitter_did = c_char_p(submitter_did.encode('utf-8')) if submitter_did is not None else None
    c_payment_method = c_char_p(payment_method.encode('utf-8'))
    c_fees_json = c_char_p(fees_json.encode('utf-8'))

    set_txn_fees_json = await do_call('indy_build_set_txn_fees_req',
                                      c_wallet_handle,
                                      c_submitter_did,
                                      c_payment_method,
                                      c_fees_json,
                                      build_set_txn_fees_req.cb)

    res = set_txn_fees_json.decode()
    logger.debug("build_set_txn_fees_req: <<< res: %r", res)
    return res


async def build_get_txn_fees_req(wallet_handle: int,
                                 submitter_did: str,
                                 payment_method: str) -> str:
    """
    Builds Indy request for getting fees for transactions in the ledger

    :param wallet_handle: wallet handle (created by open_wallet).
    :param submitter_did : (Option) DID of request sender
    :param payment_method: Payment method to use (for example, 'sov').
    :return: set_txn_fees_json: Indy request for setting fees for transactions in the ledger
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_get_txn_fees_req: >>> wallet_handle: %r, submitter_did: %r, payment_method: %r",
                 wallet_handle,
                 submitter_did,
                 payment_method)

    if not hasattr(build_get_txn_fees_req, "cb"):
        logger.debug("build_get_txn_fees_req: Creating callback")
        build_get_txn_fees_req.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_submitter_did = c_char_p(submitter_did.encode('utf-8')) if submitter_did is not None else None
    c_payment_method = c_char_p(payment_method.encode('utf-8'))

    get_txn_fees_json = await do_call('indy_build_get_txn_fees_req',
                                      c_wallet_handle,
                                      c_submitter_did,
                                      c_payment_method,
                                      build_get_txn_fees_req.cb)

    res = get_txn_fees_json.decode()
    logger.debug("build_get_txn_fees_req: <<< res: %r", res)
    return res


async def parse_get_txn_fees_response(payment_method: str,
                                      resp_json: str) -> str:
    """
    Parses response for Indy request for getting fees

    :param payment_method: Payment method to use (for example, 'sov').
    :param resp_json: response for Indy request for getting fees
    :return: fees_json: {
       txnType1: amount1,
       txnType2: amount2,
       .................
       txnTypeN: amountN,
     }
    """

    logger = logging.getLogger(__name__)
    logger.debug("parse_get_txn_fees_response: >>> payment_method: %r, resp_json: %r",
                 payment_method,
                 resp_json)

    if not hasattr(parse_get_txn_fees_response, "cb"):
        logger.debug("parse_get_txn_fees_response: Creating callback")
        parse_get_txn_fees_response.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_payment_method = c_char_p(payment_method.encode('utf-8'))
    c_resp_json = c_char_p(resp_json.encode('utf-8'))

    fees_json = await do_call('indy_parse_get_txn_fees_response',
                              c_payment_method,
                              c_resp_json,
                              parse_get_txn_fees_response.cb)

    res = fees_json.decode()
    logger.debug("parse_get_txn_fees_response: <<< res: %r", res)
    return res


async def build_verify_payment_req(wallet_handle: int,
                                   submitter_did: str,
                                   receipt: str) -> (str, str):
    """
    Builds Indy request for information to verify the payment receipt

    :param wallet_handle: wallet handle (created by open_wallet).
    :param submitter_did : (Option) DID of request sender
    :param receipt: payment receipt to verify

    :return: verify_txn_json: Indy request for verification receipt for transactions in the ledger
             payment_method: used payment method
    """

    logger = logging.getLogger(__name__)
    logger.debug("build_verify_payment_req: >>> wallet_handle: %r, submitter_did: %r, receipt: %r",
                 wallet_handle,
                 submitter_did,
                 receipt)

    if not hasattr(build_verify_payment_req, "cb"):
        logger.debug("build_verify_payment_req: Creating callback")
        build_verify_payment_req.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_submitter_did = c_char_p(submitter_did.encode('utf-8')) if submitter_did is not None else None
    c_receipt = c_char_p(receipt.encode('utf-8'))

    (verify_txn_json, payment_method) = await do_call('indy_build_verify_payment_req',
                                                      c_wallet_handle,
                                                      c_submitter_did,
                                                      c_receipt,
                                                      build_verify_payment_req.cb)
    res = (verify_txn_json.decode(), payment_method.decode())

    logger.debug("build_verify_payment_req: <<< res: %r", res)
    return res


async def parse_verify_payment_response(payment_method: str,
                                        resp_json: str) -> str:
    """
    Parses Indy response with information to verify receipt

    :param payment_method: Payment method to use (for example, 'sov').
    :param resp_json: resp_json: response for Indy request for payment txn
      Note: this param will be used to determine payment_method
    :return: receipts_json: parsed (payment method and node version agnostic) receipt verification info as json:
    {
         sources: [<str>, ]
         receipts: [ {
             recipient: <str>, // payment address of recipient
             receipt: <str>, // receipt that can be used for payment referencing and verification
             amount: <int>, // amount
         } ],
         extra: <str>, //optional data
    }
    """

    logger = logging.getLogger(__name__)
    logger.debug("parse_verify_payment_response: >>> wallet_handle: %r, payment_method: %r, resp_json: %r",
                 payment_method,
                 resp_json)

    if not hasattr(parse_verify_payment_response, "cb"):
        logger.debug("parse_verify_payment_response: Creating callback")
        parse_verify_payment_response.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_payment_method = c_char_p(payment_method.encode('utf-8'))
    c_resp_json = c_char_p(resp_json.encode('utf-8'))

    receipts_json = await do_call('indy_parse_verify_payment_response',
                                  c_payment_method,
                                  c_resp_json,
                                  parse_verify_payment_response.cb)

    res = receipts_json.decode()
    logger.debug("parse_verify_payment_response: <<< res: %r", res)
    return res
