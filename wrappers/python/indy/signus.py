from typing import Callable
from . import IndyError

class Signus(object):

    """TODO: document it"""
    async def create_and_store_my_did(command_handle: int,
                                      wallet_handle: int,
                                      did_json: str,
                                      did: str,
                                      verkey: str,
                                      pk: str) -> None:
        pass

    async def replace_keys(command_handle: int,
                           wallet_handle: int,
                           did: str,
                           identity_json: str,
                           verkey: str,
                           pk: str) -> None:
        pass

    async def store_their_did(command_handle: int,
                              wallet_handle: int,
                              identity_json: str) -> None:
        pass

    async def sign(command_handle: int,
                   wallet_handle: int,
                   did: str,
                   msg: str,
                   signature: str) -> None:
        pass

    async def verify_signature(command_handle: int,
                               wallet_handle: int,
                               did: str,
                               msg: str,
                               signature: str,
                               valid: bool) -> None:
        pass

    async def encrypt(command_handle: int,
                      wallet_handle: int,
                      did: str,
                      msg: str,
                      encrypted_msg: str) -> None:
        pass

    async def decrypt(command_handle: int,
                      wallet_handle: int,
                      did: str,
                      encrypted_msg: str,
                      decrypted_msg: str) -> None:
        pass
