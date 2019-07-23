from typing import Optional, Union

from .._command import LibindyCommand


class Crypto:

    @staticmethod
    @LibindyCommand('indy_create_key')
    async def create_key(
            wallet_handle: int,
            key_info: Union[dict, str]
    ) -> str:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_set_key_metadata')
    async def set_key_metadata(
            wallet_handle: int,
            verkey: str,
            metadata: str
    ):
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_get_key_metadata')
    async def get_key_metadata(
            wallet_handle: int,
            verkey: str
    ) -> Optional[str]:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_crypto_sign')
    async def crypto_sign(
            wallet_handle: int,
            signer_verkey: str,
            message: bytes
    ) -> bytes:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_crypto_verify')
    async def crypto_verify(
            signer_verkey: str,
            message: bytes,
            signature: bytes
    ) -> bool:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_crypto_auth_crypt')
    async def auth_crypt(
            wallet_handle: int,
            sender_verkey: str,
            recipient_verkey: str,
            message: bytes
    ) -> bytes:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_crypto_auth_decrypt')
    async def auth_decrypt(
            wallet_handle: int,
            recipient_verkey: str,
            encrypted_message: bytes
    ) -> (str, bytes):
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_crypto_anon_crypt')
    async def anon_crypt(
            recipient_verkey: str,
            message: bytes
    ) -> bytes:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_crypto_anon_decrypt')
    async def anon_decrypt(
            wallet_handle: int,
            recipient_verkey: str,
            encrypted_message: bytes
    ) -> bytes:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_pack_message')
    async def pack_message(
            wallet_handle: int,
            message: str,
            recipient_verkeys: Union[list, str],
            sender_verkey: Optional[str]
    ) -> bytes:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_unpack_message')
    async def unpack_message(
            wallet_handle: int,
            jwe: bytes
    ) -> bytes:
        """"""
        pass
