from .libindy import do_call, create_cb

from ctypes import *

import logging


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


async def crypto_sign(wallet_handle: int,
                      my_vk: str,
                      msg: bytes) -> bytes:
    """
    Signs a message with a key.

    Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey) for specific DID.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param my_vk:  id (verkey) of my key. The key must be created by calling indy_create_key or
    indy_create_and_store_my_did
    :param msg: a message to be signed
    :return: a signature string
    """

    logger = logging.getLogger(__name__)
    logger.debug("crypto_sign: >>> wallet_handle: %r, my_vk: %r, msg: %r",
                 wallet_handle,
                 my_vk,
                 msg)

    def transform_cb(arr_ptr: POINTER(c_uint8), arr_len: c_uint32):
        return bytes(arr_ptr[:arr_len]),

    if not hasattr(crypto_sign, "cb"):
        logger.debug("crypto_sign: Creating callback")
        crypto_sign.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, POINTER(c_uint8), c_uint32), transform_cb)

    c_wallet_handle = c_int32(wallet_handle)
    c_my_vk = c_char_p(my_vk.encode('utf-8'))
    c_msg_len = c_uint32(len(msg))

    signature = await do_call('indy_crypto_sign',
                              c_wallet_handle,
                              c_my_vk,
                              msg,
                              c_msg_len,
                              crypto_sign.cb)

    logger.debug("crypto_sign: <<< res: %r", signature)
    return signature


async def crypto_verify(their_vk: str,
                        msg: bytes,
                        signature: bytes) -> bool:
    """
    Verify a signature with a verkey.

    Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey) for specific DID.

    :param their_vk: verkey to use
    :param msg:  message to be signed
    :param signature: a signature to be verified
    :return: valid: true - if signature is valid, false - otherwise
    """

    logger = logging.getLogger(__name__)
    logger.debug("crypto_verify: >>> my_vk: %r, signed_msg: %r",
                 their_vk,
                 msg,
                 signature)

    if not hasattr(crypto_verify, "cb"):
        logger.debug("crypto_verify: Creating callback")
        crypto_verify.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_bool))

    c_their_vk = c_char_p(their_vk.encode('utf-8'))
    c_msg_len = c_uint32(len(msg))
    c_signature_len = c_uint32(len(signature))

    res = await do_call('indy_crypto_verify',
                        c_their_vk,
                        msg,
                        c_msg_len,
                        signature,
                        c_signature_len,
                        crypto_verify.cb)

    logger.debug("crypto_verify: <<< res: %r", res)
    return res


async def crypto_box(wallet_handle: int,
                     my_vk: str,
                     their_vk: str,
                     msg: bytes) -> (bytes, bytes):
    """
    Encrypt a message by authenticated-encryption scheme.

    Sender can encrypt a confidential message specifically for Recipient, using Sender's public key.
    Using Recipient's public key, Sender can compute a shared secret key.
    Using Sender's public key and his secret key, Recipient can compute the exact same shared secret key.
    That shared secret key can be used to verify that the encrypted message was not tampered with,
    before eventually decrypting it.

    Recipient only needs Sender's public key, the nonce and the ciphertext to peform decryption.
    The nonce doesn't have to be confidential.

    Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
    for specific DID.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param my_vk: id (verkey) of my key. The key must be created by calling indy_create_key or
    indy_create_and_store_my_did
    :param their_vk: id (verkey) of their key
    :param msg: a message to be signed
    :return: an encrypted message and nonce
    """

    logger = logging.getLogger(__name__)
    logger.debug("crypto_box: >>> wallet_handle: %r,my_vk: %r, their_vk: %r, msg: %r",
                 wallet_handle,
                 my_vk,
                 their_vk,
                 msg)

    def transform_cb(arr_ptr: POINTER(c_uint8), arr_len: c_uint32, arr_ptr2: POINTER(c_uint8), arr_len2: c_uint32):
        return bytes(arr_ptr[:arr_len]), bytes(arr_ptr2[:arr_len2])

    if not hasattr(crypto_box, "cb"):
        logger.debug("crypto_box: Creating callback")
        crypto_box.cb = create_cb(
            CFUNCTYPE(None, c_int32, c_int32, POINTER(c_uint8), c_uint32, POINTER(c_uint8),
                      c_uint32), transform_cb)

    c_wallet_handle = c_int32(wallet_handle)
    c_my_vk = c_char_p(my_vk.encode('utf-8'))
    c_their_vk = c_char_p(their_vk.encode('utf-8'))
    c_msg_len = c_uint32(len(msg))

    encrypted_message, nonce = await do_call('indy_crypto_box',
                                             c_wallet_handle,
                                             c_my_vk,
                                             c_their_vk,
                                             msg,
                                             c_msg_len,
                                             crypto_box.cb)
    res = (encrypted_message, nonce)

    logger.debug("crypto_box: <<< res: %r", res)
    return res


async def crypto_box_open(wallet_handle: int,
                          my_vk: str,
                          their_vk: str,
                          encrypted_msg: bytes,
                          nonce: bytes) -> bytes:
    """
    Decrypt a message by authenticated-encryption scheme.

    Sender can encrypt a confidential message specifically for Recipient, using Sender's public key.
    Using Recipient's public key, Sender can compute a shared secret key.
    Using Sender's public key and his secret key, Recipient can compute the exact same shared secret key.
    That shared secret key can be used to verify that the encrypted message was not tampered with,
    before eventually decrypting it.

    Recipient only needs Sender's public key, the nonce and the ciphertext to peform decryption.
    Еhe nonce doesn't have to be confidential.

    Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
    for specific DID.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param my_vk: id (verkey) of my key. The key must be created by calling indy_create_key or
    indy_create_and_store_my_did
    :param their_vk: id (verkey) of their key
    :param encrypted_msg: encrypted message
    :param nonce: nonce that encrypted message
    :return: decrypted message
    """

    logger = logging.getLogger(__name__)
    logger.debug("crypto_box_open: >>> wallet_handle: %r, my_did: %r, did: %r, encrypted_msg: %r, nonce: %r",
                 wallet_handle,
                 my_vk,
                 their_vk,
                 encrypted_msg,
                 nonce)

    def transform_cb(arr_ptr: POINTER(c_uint8), arr_len: c_uint32):
        return bytes(arr_ptr[:arr_len]),

    if not hasattr(crypto_box_open, "cb"):
        logger.debug("crypto_box_open: Creating callback")
        crypto_box_open.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, POINTER(c_uint8), c_uint32),
                                       transform_cb)

    c_wallet_handle = c_int32(wallet_handle)
    c_my_vk = c_char_p(my_vk.encode('utf-8'))
    c_their_vk = c_char_p(their_vk.encode('utf-8'))
    c_encrypted_msg_len = c_uint32(len(encrypted_msg))
    c_nonce_len = c_uint32(len(nonce))

    decrypted_message = await do_call('indy_crypto_box_open',
                                      c_wallet_handle,
                                      c_my_vk,
                                      c_their_vk,
                                      bytes(encrypted_msg),
                                      c_encrypted_msg_len,
                                      bytes(nonce),
                                      c_nonce_len,
                                      crypto_box_open.cb)

    logger.debug("crypto_box_open: <<< res: %r", decrypted_message)
    return decrypted_message


async def crypto_box_seal(their_vk: str,
                          msg: bytes) -> (bytes, bytes):
    """
    Encrypts a message by anonymous-encryption scheme.

    Sealed boxes are designed to anonymously send messages to a Recipient given its public key.
    Only the Recipient can decrypt these messages, using its private key.
    While the Recipient can verify the integrity of the message, it cannot verify the identity of the Sender.

    Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
    for specific DID.

    :param their_vk: id (verkey) of their key
    :param msg: a message to be signed
    :return: an encrypted message and nonce
    """

    logger = logging.getLogger(__name__)
    logger.debug("crypto_box_seal: >>> their_vk: %r, msg: %r",
                 their_vk,
                 msg)

    def transform_cb(arr_ptr: POINTER(c_uint8), arr_len: c_uint32):
        return bytes(arr_ptr[:arr_len])

    if not hasattr(crypto_box_seal, "cb"):
        logger.debug("crypto_box_seal: Creating callback")
        crypto_box_seal.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, POINTER(c_uint8), c_uint32), transform_cb)

    c_their_vk = c_char_p(their_vk.encode('utf-8'))
    c_msg_len = c_uint32(len(msg))

    encrypted_message = await do_call('indy_crypto_box_seal',
                                      c_their_vk,
                                      msg,
                                      c_msg_len,
                                      crypto_box_seal.cb)
    res = encrypted_message
    logger.debug("crypto_box_seal: <<< res: %r", res)
    return res


async def crypto_box_seal_open(wallet_handle: int,
                               my_vk: str,
                               encrypted_msg: bytes) -> bytes:
    """
    Decrypts a message by anonymous-encryption scheme.

    Sealed boxes are designed to anonymously send messages to a Recipient given its public key.
    Only the Recipient can decrypt these messages, using its private key.
    While the Recipient can verify the integrity of the message, it cannot verify the identity of the Sender.

    Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
    for specific DID.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param my_vk: id (verkey) of my key. The key must be created by calling indy_create_key or
     indy_create_and_store_my_did
    :param encrypted_msg: encrypted message
    :return: decrypted message
    """

    logger = logging.getLogger(__name__)
    logger.debug("crypto_box_seal_open: >>> wallet_handle: %r, my_vk: %r, encrypted_msg: %r",
                 wallet_handle,
                 my_vk,
                 encrypted_msg)

    def transform_cb(arr_ptr: POINTER(c_uint8), arr_len: c_uint32):
        return bytes(arr_ptr[:arr_len]),

    if not hasattr(crypto_box_seal_open, "cb"):
        logger.debug("crypto_box_seal_open: Creating callback")
        crypto_box_seal_open.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, POINTER(c_uint8), c_uint32), transform_cb)

    c_wallet_handle = c_int32(wallet_handle)
    c_my_vk = c_char_p(my_vk.encode('utf-8'))
    c_encrypted_msg_len = c_uint32(len(encrypted_msg))
    decrypted_message = await do_call('indy_crypto_box_seal_open',
                                      c_wallet_handle,
                                      c_my_vk,
                                      bytes(encrypted_msg),
                                      c_encrypted_msg_len,
                                      crypto_box_seal_open.cb)
    logger.debug("crypto_box_seal_open: <<< res: %r", decrypted_message)
    return decrypted_message
