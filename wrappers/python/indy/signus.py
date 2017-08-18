from .libindy import do_call, create_cb

from ctypes import *

import logging


async def create_and_store_my_did(wallet_handle: int,
                                  did_json: str) -> (str, str, str):
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
                    if not provided and cid param is false then the first 16 bit of the verkey will be used as a new DID;
                    if not provided and cid is true then the full verkey will be used as a new DID;
                    if provided, then keys will be replaced - key rotation use case)
            "seed": string, (optional; if not provide then a random one will be created)
            "crypto_type": string, (optional; if not set then ed25519 curve is used;
                      currently only 'ed25519' value is supported for this field)
            "cid": bool, (optional; if not set then false is used;)
        }
    :return: DID, verkey (for verification of signature) and public_key (for decryption)
    """

    logger = logging.getLogger(__name__)
    logger.debug("create_and_store_my_did: >>> wallet_handle: %r, did_json: %r",
                 wallet_handle,
                 did_json)

    if not hasattr(create_and_store_my_did, "cb"):
        logger.debug("create_wallet: Creating callback")
        create_and_store_my_did.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p, c_char_p, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_did_json = c_char_p(did_json.encode('utf-8'))

    did, verkey, pk = await do_call('indy_create_and_store_my_did',
                                    c_wallet_handle,
                                    c_did_json,
                                    create_and_store_my_did.cb)

    res = (did.decode(), verkey.decode(), pk.decode())

    logger.debug("create_and_store_my_did: <<< res: %r", res)
    return res


async def replace_keys(wallet_handle: int,
                       did: str,
                       identity_json: str) -> (str, str):
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
    :return: verkey (for verification of signature) and public_key (for decryption)
    """

    logger = logging.getLogger(__name__)
    logger.debug("replace_keys: >>> wallet_handle: %r, did: %r, identity_json: %r",
                 wallet_handle,
                 did,
                 identity_json)

    if not hasattr(replace_keys, "cb"):
        logger.debug("replace_keys: Creating callback")
        replace_keys.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_did = c_char_p(did.encode('utf-8'))
    c_identity_json = c_char_p(identity_json.encode('utf-8'))

    verkey, pk = await do_call('indy_replace_keys',
                               c_wallet_handle,
                               c_did,
                               c_identity_json,
                               replace_keys.cb)

    res = (verkey.decode(), pk.decode())

    logger.debug("replace_keys: <<< res: %r", res)
    return res


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


async def sign(wallet_handle: int,
               did: str,
               msg: bytes) -> bytes:
    """
    Signs a message by a signing key associated with my DID. The DID with a signing key
    must be already created and stored in a secured wallet (see create_and_store_my_identity)

    :param wallet_handle: wallet handler (created by open_wallet).
    :param did: signing DID
    :param msg: a message to be signed
    :return: a signature string
    """

    logger = logging.getLogger(__name__)
    logger.debug("sign: >>> wallet_handle: %r, did: %r, msg: %r",
                 wallet_handle,
                 did,
                 msg)

    def transform_cb(arr_ptr: POINTER(c_uint8), arr_len: c_uint32):
        return bytes(arr_ptr[:arr_len]),

    if not hasattr(sign, "cb"):
        logger.debug("sign: Creating callback")
        sign.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, POINTER(c_uint8), c_uint32), transform_cb)

    c_wallet_handle = c_int32(wallet_handle)
    c_did = c_char_p(did.encode('utf-8'))
    c_msg_len = c_uint32(len(msg))

    signature = await do_call('indy_sign',
                              c_wallet_handle,
                              c_did,
                              msg,
                              c_msg_len,
                              sign.cb)

    logger.debug("sign: <<< res: %r", signature)
    return signature


async def verify_signature(wallet_handle: int,
                           pool_handle: int,
                           did: str,
                           msg: bytes,
                           signature: bytes) -> bool:
    """
    Verify a signature created by a key associated with a DID.
    If a secure wallet doesn't contain a verkey associated with the given DID,
    then verkey is read from the Ledger.
    Otherwise either an existing verkey from wallet is used (see wallet_store_their_identity),
    or it checks the Ledger (according to freshness settings set during initialization)
    whether verkey is still the same and updates verkey for the DID if needed.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param pool_handle: pool handle.
    :param did: DID that signed the message
    :param msg: message
    :param signature: a signature to be verified
    :return: valid: true - if signature is valid, false - otherwise
    """

    logger = logging.getLogger(__name__)
    logger.debug("verify_signature: >>> wallet_handle: %r, pool_handle: %r, did: %r, signed_msg: %r",
                 wallet_handle,
                 pool_handle,
                 did,
                 msg,
                 signature)

    if not hasattr(verify_signature, "cb"):
        logger.debug("verify_signature: Creating callback")
        verify_signature.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_bool))

    c_wallet_handle = c_int32(wallet_handle)
    c_pool_handle = c_int32(pool_handle)
    c_did = c_char_p(did.encode('utf-8'))
    c_msg_len = c_uint32(len(msg))
    c_signature_len = c_uint32(len(signature))

    res = await do_call('indy_verify_signature',
                        c_wallet_handle,
                        c_pool_handle,
                        c_did,
                        msg,
                        c_msg_len,
                        signature,
                        c_signature_len,
                        verify_signature.cb)

    logger.debug("verify_signature: <<< res: %r", res)
    return res


async def encrypt(wallet_handle: int,
                  pool_handle: int,
                  my_did: str,
                  did: str,
                  msg: bytes) -> (bytes, bytes):
    """
    Encrypts a message by a public key associated with a DID.
    If a secure wallet doesn't contain a public key associated with the given DID,
    then the public key is read from the Ledger.
    Otherwise either an existing public key from wallet is used (see wallet_store_their_identity),
    or it checks the Ledger (according to freshness settings set during initialization)
    whether public key is still the same and updates public key for the DID if needed.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param pool_handle: pool handle.
    :param my_did: encrypting DID
    :param did: encrypting DID
    :param msg: a message to be signed
    :return: an encrypted message and nonce
    """

    logger = logging.getLogger(__name__)
    logger.debug("encrypt: >>> wallet_handle: %r, pool_handle: %r, my_did: %r, did: %r, msg: %r",
                 wallet_handle,
                 pool_handle,
                 my_did,
                 did,
                 msg)

    def transform_cb(arr_ptr: POINTER(c_uint8), arr_len: c_uint32, arr_ptr2: POINTER(c_uint8), arr_len2: c_uint32):
        return bytes(arr_ptr[:arr_len]), bytes(arr_ptr2[:arr_len2])

    if not hasattr(encrypt, "cb"):
        logger.debug("encrypt: Creating callback")
        encrypt.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, POINTER(c_uint8), c_uint32, POINTER(c_uint8),
                                         c_uint32), transform_cb)

    c_wallet_handle = c_int32(wallet_handle)
    c_pool_handle = c_int32(pool_handle)
    c_my_did = c_char_p(my_did.encode('utf-8'))
    c_did = c_char_p(did.encode('utf-8'))
    c_msg_len = c_uint32(len(msg))

    encrypted_message, nonce = await do_call('indy_encrypt',
                                             c_wallet_handle,
                                             c_pool_handle,
                                             c_my_did,
                                             c_did,
                                             msg,
                                             c_msg_len,
                                             encrypt.cb)
    res = (encrypted_message, nonce)

    logger.debug("encrypt: <<< res: %r", res)
    return res


async def decrypt(wallet_handle: int,
                  my_did: str,
                  did: str,
                  encrypted_msg: bytes,
                  nonce: bytes) -> bytes:
    """
    Decrypts a message encrypted by a public key associated with my DID.
    The DID with a secret key must be already created and
    stored in a secured wallet (see wallet_create_and_store_my_identity)

    :param wallet_handle: wallet handler (created by open_wallet).
    :param my_did: DID
    :param did: DID that signed the message
    :param encrypted_msg: encrypted message
    :param nonce: nonce that encrypted message
    :return: decrypted message
    """

    logger = logging.getLogger(__name__)
    logger.debug("decrypt: >>> wallet_handle: %r, my_did: %r, did: %r, encrypted_msg: %r, nonce: %r",
                 wallet_handle,
                 my_did,
                 did,
                 encrypted_msg,
                 nonce)

    def transform_cb(arr_ptr: POINTER(c_uint8), arr_len: c_uint32):
        return bytes(arr_ptr[:arr_len]),

    if not hasattr(decrypt, "cb"):
        logger.debug("decrypt: Creating callback")
        decrypt.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, POINTER(c_uint8), c_uint32), transform_cb)

    c_wallet_handle = c_int32(wallet_handle)
    c_my_did = c_char_p(my_did.encode('utf-8'))
    c_did = c_char_p(did.encode('utf-8'))
    c_encrypted_msg_len = c_uint32(len(encrypted_msg))
    c_nonce_len = c_uint32(len(nonce))

    decrypted_message = await do_call('indy_decrypt',
                                      c_wallet_handle,
                                      c_my_did,
                                      c_did,
                                      encrypted_msg,
                                      c_encrypted_msg_len,
                                      nonce,
                                      c_nonce_len,
                                      decrypt.cb)

    logger.debug("decrypt: <<< res: %r", decrypted_message)
    return decrypted_message
